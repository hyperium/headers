use std::cmp::Ordering;
use std::convert::TryFrom;
use std::marker::PhantomData;

use HeaderValue;
use itertools::Itertools;
use util::{FlatCsv, TryFromValues};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct QualityValue<QualSep = SemiQ> {
    csv: FlatCsv,
    _marker: PhantomData<QualSep>,
}

pub(crate) trait QualityDelimiter {
    const STR: &'static str;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum SemiQ {}

impl QualityDelimiter for SemiQ {
    const STR: &'static str = ";q=";
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

impl<Delm: QualityDelimiter> TryFromValues for QualityValue<Delm> {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where 
        I: Iterator<Item = &'i HeaderValue>,
    {
        let flat: FlatCsv = values.collect();
        Ok(QualityValue::from(flat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::flat_csv::Comma;
    use HeaderValue;

    #[test]
    fn multiple_qualities() {
        let val = HeaderValue::from_static("gzip;q=1, br;q=0.8");
        let csv = FlatCsv::<Comma>::from(val);
        let qual = QualityValue::<SemiQ>::from(csv);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_qualities_wrong_order() {
        let val = HeaderValue::from_static("br;q=0.8, gzip;q=1");
        let csv = FlatCsv::<Comma>::from(val);
        let qual = QualityValue::<SemiQ>::from(csv);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_values() {
        let val = HeaderValue::from_static("deflate, gzip;q=1, br;q=0.8");
        let csv = FlatCsv::<Comma>::from(val);
        let qual = QualityValue::<SemiQ>::from(csv);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("deflate"));
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), None);
    }

    #[test]
    fn multiple_values_wrong_order() {
        let val = HeaderValue::from_static("deflate, br;q=0.8, gzip;q=1, *;q=0.1");
        let csv = FlatCsv::<Comma>::from(val);
        let qual = QualityValue::<SemiQ>::from(csv);

        let mut values = qual.iter();
        assert_eq!(values.next(), Some("deflate"));
        assert_eq!(values.next(), Some("gzip"));
        assert_eq!(values.next(), Some("br"));
        assert_eq!(values.next(), Some("*"));
        assert_eq!(values.next(), None);
    }
}
