use self::sealed::SemiQ;
use std::marker::PhantomData;
use util::FlatCsv;

/// A CSV list that respects the Quality Values syntax defined in
/// [RFC7321](https://tools.ietf.org/html/rfc7231#section-5.3.1)
///
/// Many of the request header fields for proactive negotiation use a
/// common parameter, named "q" (case-insensitive), to assign a relative
/// "weight" to the preference for that associated kind of content.  This
/// weight is referred to as a "quality value" (or "qvalue") because the
/// same parameter name is often used within server configurations to
/// assign a weight to the relative quality of the various
/// representations that can be selected for a resource.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QualityValue<QualSep = SemiQ> {
    csv: FlatCsv,
    _marker: PhantomData<QualSep>,
}

mod sealed {
    use super::QualityValue;
    use std::cmp::Ordering;
    use std::convert::{From, TryFrom};
    use std::marker::PhantomData;

    use itertools::Itertools;
    use util::{FlatCsv, TryFromValues};
    use HeaderValue;

    pub trait QualityDelimiter {
        const STR: &'static str;
    }

    /// enum that represents the ';q=' delimiter
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum SemiQ {}

    impl QualityDelimiter for SemiQ {
        const STR: &'static str = ";q=";
    }

    /// enum that represents the ';level=' delimiter (extremely rare)
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum SemiLevel {}

    impl QualityDelimiter for SemiLevel {
        const STR: &'static str = ";level=";
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct QualityMeta<'a, Sep = SemiQ> {
        pub data: &'a str,
        pub quality: u16,
        _marker: PhantomData<Sep>,
    }

    impl<Delm: QualityDelimiter + Ord> Ord for QualityMeta<'_, Delm> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.quality.cmp(&self.quality)
        }
    }

    impl<Delm: QualityDelimiter + Ord> PartialOrd for QualityMeta<'_, Delm> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<'a, Delm: QualityDelimiter> TryFrom<&'a str> for QualityMeta<'a, Delm> {
        type Error = ::Error;

        fn try_from(val: &'a str) -> Result<Self, ::Error> {
            let mut parts: Vec<&str> = val.split(Delm::STR).collect();

            match (parts.pop(), parts.pop()) {
                (Some(qual), Some(data)) => {
                    let parsed: f32 = qual.parse().map_err(|_| ::Error::invalid())?;
                    let quality = (parsed * 1000_f32) as u16;

                    Ok(QualityMeta {
                        data,
                        quality,
                        _marker: PhantomData,
                    })
                }
                // No deliter present, assign a quality value of 1
                (Some(data), None) => Ok(QualityMeta {
                    data,
                    quality: 1000_u16,
                    _marker: PhantomData,
                }),
                _ => Err(::Error::invalid()),
            }
        }
    }

    impl<Delm: QualityDelimiter + Ord> QualityValue<Delm> {
        pub(crate) fn iter(&self) -> impl Iterator<Item = &str> {
            self.csv
                .iter()
                .map(|v| QualityMeta::<Delm>::try_from(v).unwrap())
                .into_iter()
                .sorted()
                .map(|pair| pair.data)
                .into_iter()
        }
    }

    impl<Delm: QualityDelimiter> From<FlatCsv> for QualityValue<Delm> {
        fn from(csv: FlatCsv) -> Self {
            QualityValue {
                csv,
                _marker: PhantomData,
            }
        }
    }

    impl<Delm: QualityDelimiter, F: Into<f32>> TryFrom<(&str, F)> for QualityValue<Delm> {
        type Error = ::Error;

        fn try_from(pair: (&str, F)) -> Result<Self, ::Error> {
            let value = HeaderValue::try_from(format!("{}{}{}", pair.0, Delm::STR, pair.1.into()))
                .map_err(|_e| ::Error::invalid())?;
            Ok(QualityValue {
                csv: value.into(),
                _marker: PhantomData,
            })
        }
    }

    impl<Delm> From<HeaderValue> for QualityValue<Delm> {
        fn from(value: HeaderValue) -> Self {
            QualityValue {
                csv: value.into(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a, Delm> From<&'a QualityValue<Delm>> for HeaderValue {
        fn from(qual: &'a QualityValue<Delm>) -> HeaderValue {
            qual.csv.value.clone()
        }
    }

    impl<Delm> From<QualityValue<Delm>> for HeaderValue {
        fn from(qual: QualityValue<Delm>) -> HeaderValue {
            qual.csv.value
        }
    }

    impl<Delm: QualityDelimiter> TryFromValues for QualityValue<Delm> {
        fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
        where
            I: Iterator<Item = &'i HeaderValue>,
        {
            let flat: FlatCsv = values.collect();
            Ok(QualityValue::from(flat))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        sealed::{SemiLevel, SemiQ},
        QualityValue,
    };
    use HeaderValue;

    #[test]
    fn multiple_qualities() {
        let val = HeaderValue::from_static("gzip;q=1, br;q=0.8");
        let qual = QualityValue::<SemiQ>::from(val);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_qualities_wrong_order() {
        let val = HeaderValue::from_static("br;q=0.8, gzip;q=1.0");
        let qual = QualityValue::<SemiQ>::from(val);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_values() {
        let val = HeaderValue::from_static("deflate, gzip;q=1, br;q=0.8");
        let qual = QualityValue::<SemiQ>::from(val);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("deflate"));
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_values_wrong_order() {
        let val = HeaderValue::from_static("deflate, br;q=0.8, gzip;q=1, *;q=0.1");
        let qual = QualityValue::<SemiQ>::from(val);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("deflate"));
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), Some("*"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn alternate_delimiter() {
        let val = HeaderValue::from_static("deflate, br;level=0.8, gzip;level=1");
        let qual = QualityValue::<SemiLevel>::from(val);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("deflate"));
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }
}
