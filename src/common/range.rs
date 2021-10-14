use std::{
    num::NonZeroU64,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

/// `Range` header, defined in [RFC7233](https://tools.ietf.org/html/rfc7233#section-3.1)
///
/// The "Range" header field on a GET request modifies the method
/// semantics to request transfer of only one or more subranges of the
/// selected representation data, rather than the entire selected
/// representation data.
///
/// # ABNF
///
/// ```text
/// Range = byte-ranges-specifier / other-ranges-specifier
/// other-ranges-specifier = other-range-unit "=" other-range-set
/// other-range-set = 1*VCHAR
///
/// byte-ranges-specifier = bytes-unit "=" byte-range-set
/// byte-range-set = 1#(byte-range-spec / suffix-byte-range-spec)
///
/// bytes-unit = "bytes"
///
/// byte-range-spec = first-byte-pos "-" [last-byte-pos]
/// first-byte-pos = 1*DIGIT
/// last-byte-pos = 1*DIGIT
///
/// suffix-byte-range-spec = "-" suffix-length
/// suffix-length = 1*DIGIT
/// ```
///
/// # Example values
///
/// * `bytes=1000-`
/// * `bytes=-2000`
/// * `bytes=0-1,30-40`
/// * `bytes=0-10,20-90,-100`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::{Range, ByteRangeBuilder};
///
/// let range : Range = ByteRangeBuilder::new()
///     .range(0..20).unwrap()
///     .range(1000..1700).unwrap()
///     .suffix(300).unwrap()
///     .finish().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Range {
    /// Since representation data is transferred in payloads as a sequence of
    /// octets, a byte range is a meaningful substructure for any
    /// representation transferable over HTTP (Section 3 of [RFC7231]). The
    /// "bytes" range unit is defined for expressing subranges of the data's
    /// octet sequence.
    ///
    /// These byte ranges are exactly as they came from the client. You may
    /// wish to coalesce them in case of overlaps, as described in [section
    /// 4.1 of RFC7233](https://tools.ietf.org/html/rfc7233#section-4.1).
    Bytes(Vec<ByteRangeSpec>),
    /// Range units are intended to be extensible. New range units ought to be
    /// registered with IANA, as defined in [Section 5.1 of RFC 7233](2).
    ///
    /// `bytes` is the only [registered](1) unit at the moment, so all other
    /// units will end up in this variant, `Unregistered`. For those other
    /// units, no concrete format for the `other-ranges-set` is given in RFC
    /// 7233, so additional parsing has to be done by the consumer of the
    /// unregistered units.
    ///
    /// [1]: https://www.iana.org/assignments/http-parameters/http-parameters.xhtml#range-units
    /// [2]: https://datatracker.ietf.org/doc/html/rfc7233#section-5.1
    Unregistered {
        /// The unit for other ranges. This has to be a `token`, as defined in
        /// [section 3.2.6 of RFC 7230](1).
        ///
        /// [1]: https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6
        unit: String,
        /// Representation of the range set for the custom, unregistered unit.
        /// This can be a string made up out of any printable ASCII character.
        set: String,
    },
}

/// A single byte range, which can either be a range or a suffix.
#[derive(Clone, Debug, PartialEq)]
pub enum ByteRangeSpec {
    /// For a range, indices of the first and last byte are given.
    FromTo(u64, u64),
    /// Remainder of the resource, starting from the byte index.
    AllFrom(u64),
    /// For a suffix, only the length of the suffix is given, and the server is prompted to return
    /// the last bytes only.
    Last(NonZeroU64),
}

impl Range {
    /// Convenience method to convert into [RangeBounds], if this is a set of end-inclusive satisfiable byte ranges.
    /// If it isn't, then this will return `Err`.
    pub fn to_satisfiable_range_bounds(
        &self,
        len: u64,
    ) -> Result<Vec<impl RangeBounds<u64>>, ::Error> {
        if let Self::Bytes(ranges) = self {
            ranges
                .iter()
                .map(|spec| spec.to_satisfiable_range_bounds(len))
                .collect()
        } else {
            Err(::Error::invalid())
        }
    }
}

impl ByteRangeSpec {
    /// Given the full length of the entity, attempt to normalize the byte
    /// range into an satisfiable end-inclusive (from, to) range.
    ///
    /// The resulting range is guaranteed to be a satisfiable range within
    /// the bounds of `0 <= from <= to < full_length`.
    ///
    /// If the byte range is deemed unsatisfiable, `Err` is returned. An
    /// unsatisfiable range is generally cause for a server to either reject
    /// the client request with a `416 Range Not Satisfiable` status code,
    /// or to simply ignore the range header and serve the full entity using
    /// a `200 OK` status code.
    ///
    /// This function closely follows [RFC 7233][1] section 2.1.
    /// As such, it considers ranges to be satisfiable if they meet the
    /// following conditions:
    ///
    /// > If a valid byte-range-set includes at least one byte-range-spec with
    /// a first-byte-pos that is less than the current length of the
    /// representation, or at least one suffix-byte-range-spec with a
    /// non-zero suffix-length, then the byte-range-set is satisfiable.
    /// Otherwise, the byte-range-set is unsatisfiable.
    ///
    /// The function also computes remainder ranges based on the RFC:
    ///
    /// > If the last-byte-pos value is
    /// absent, or if the value is greater than or equal to the current
    /// length of the representation data, the byte range is interpreted as
    /// the remainder of the representation (i.e., the server replaces the
    /// value of last-byte-pos with a value that is one less than the current
    /// length of the selected representation).
    ///
    /// [1]: https://tools.ietf.org/html/rfc7233
    pub fn to_satisfiable_range_bounds(&self, len: u64) -> Result<impl RangeBounds<u64>, ::Error> {
        // If the full length is zero, there is no satisfiable end-inclusive range.
        if len == 0 {
            return Err(::Error::invalid());
        }
        match self {
            ByteRangeSpec::FromTo(first, last) => {
                // If the index of the first byte is beyond the end, or after
                // the index of the first byte, the range is not satisfiable
                if *first >= len || first > last {
                    Err(::Error::invalid())
                } else {
                    // Clamp index of the last requested by to the last byte in
                    // the resource
                    Ok((
                        Bound::Included(*first),
                        Bound::Included(u64::min(*last, len - 1)),
                    ))
                }
            }
            ByteRangeSpec::AllFrom(first) => {
                if *first >= len {
                    return Err(::Error::invalid());
                }
                Ok((Bound::Included(*first), Bound::Excluded(len)))
            }
            ByteRangeSpec::Last(suf_len) => {
                // If the requested suffix length is longer than the resource, set the starting
                // bound to the start of the resource
                Ok((
                    Bound::Included(len.saturating_sub(suf_len.get())),
                    Bound::Excluded(len),
                ))
            }
        }
    }
}

impl ::Header for Range {
    fn name() -> &'static ::HeaderName {
        &::http::header::RANGE
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|val| val.to_str().ok()?.parse().ok())
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        let value = ::HeaderValue::from_str(&format!("{}", self)).unwrap();
        values.extend(::std::iter::once(value));
    }
}

/// Builder for byte range headers
#[derive(Clone, Debug)]
pub struct ByteRangeBuilder {
    inner: Vec<ByteRangeSpec>,
}

impl Default for ByteRangeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteRangeBuilder {
    /// Create a new byte range builder
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Add a range to the header. Fails if the end bound of the range
    /// is before the start bound.
    pub fn range(mut self, bounds: impl RangeBounds<u64>) -> Result<Self, InvalidRange> {
        match reduce_bounds(bounds) {
            (first, Some(last)) => {
                if first > last {
                    return Err(err());
                }
                self.inner.push(ByteRangeSpec::FromTo(first, last));
            }
            (first, None) => {
                self.inner.push(ByteRangeSpec::AllFrom(first));
            }
        };
        Ok(self)
    }

    /// Add a suffix to the header. Fails if the suffix length is zero.
    pub fn suffix(mut self, len: u64) -> Result<Self, InvalidRange> {
        let len = match NonZeroU64::new(len) {
            Some(len) => len,
            None => return Err(err()),
        };

        self.inner.push(ByteRangeSpec::Last(len));
        Ok(self)
    }

    /// Finish up the header. Fails if no ranges or suffixes were added.
    pub fn finish(self) -> Result<Range, InvalidRange> {
        if self.inner.is_empty() {
            return Err(err());
        }
        Ok(Range::Bytes(self.inner))
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Range::Bytes(ranges) => {
                write!(f, "bytes=")?;
                for (i, range) in ranges.iter().enumerate() {
                    if i != 0 {
                        f.write_str(",")?;
                    }
                    std::fmt::Display::fmt(range, f)?;
                }
                Ok(())
            }
            Range::Unregistered { unit, set } => write!(f, "{}={}", unit, set),
        }
    }
}

impl FromStr for Range {
    type Err = InvalidRange;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, '=');

        match (iter.next(), iter.next()) {
            (Some("bytes"), Some(set)) => {
                let specs: Result<Vec<ByteRangeSpec>, Self::Err> =
                    set.split(',').map(|spec| spec.parse()).collect();
                match specs {
                    Ok(specs) if !specs.is_empty() => Ok(Self::Bytes(specs)),
                    _ => Err(err()),
                }
            }
            (Some(unit), Some(set)) if !unit.is_empty() && !set.is_empty() => {
                Ok(Self::Unregistered {
                    unit: unit.to_string(),
                    set: set.to_string(),
                })
            }
            _ => Err(err()),
        }
    }
}

impl std::fmt::Display for ByteRangeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ByteRangeSpec::FromTo(first, last) => write!(f, "{}-{}", first, last),
            ByteRangeSpec::Last(len) => write!(f, "-{}", len),
            ByteRangeSpec::AllFrom(first) => write!(f, "{}-", first),
        }
    }
}

impl FromStr for ByteRangeSpec {
    type Err = InvalidRange;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '-');

        match (parts.next(), parts.next()) {
            (Some(""), Some(end)) => end.parse().or(Err(err())).map(ByteRangeSpec::Last),
            (Some(start), Some("")) => start.parse().or(Err(err())).map(ByteRangeSpec::AllFrom),
            (Some(start), Some(end)) => match (start.parse(), end.parse()) {
                (Ok(start), Ok(end)) if start <= end => Ok(ByteRangeSpec::FromTo(start, end)),
                _ => Err(err()),
            },
            _ => Err(err()),
        }
    }
}
error_type!(InvalidRange);

fn err() -> InvalidRange {
    InvalidRange { _inner: () }
}

fn reduce_bounds(bounds: impl RangeBounds<u64>) -> (u64, Option<u64>) {
    (
        match bounds.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => n + 1,
            Bound::Unbounded => 0,
        },
        match bounds.end_bound() {
            Bound::Included(n) => Some(*n),
            Bound::Excluded(n) => Some(n - 1),
            Bound::Unbounded => None,
        },
    )
}

#[cfg(test)]
mod test {
    use std::num::NonZeroU64;

    use crate::common::test_decode;
    use crate::{ByteRangeBuilder, ByteRangeSpec, Range};

    use super::reduce_bounds;

    #[test]
    fn test_from_str_byte_range_spec() {
        let r: ByteRangeSpec = "1-100".parse().unwrap();
        let r2 = ByteRangeSpec::FromTo(1, 100);
        assert_eq!(r, r2);

        let r: ByteRangeSpec = "200-".parse().unwrap();
        let r2 = ByteRangeSpec::AllFrom(200);
        assert_eq!(r, r2);

        let r: ByteRangeSpec = "-100".parse().unwrap();
        let r2 = ByteRangeSpec::Last(NonZeroU64::new(100).unwrap());
        assert_eq!(r, r2);
    }

    #[test]
    fn test_from_str_range() {
        let r: Range = "bytes=1-100".parse().unwrap();
        let r2 = ByteRangeBuilder::new()
            .range(1..=100)
            .unwrap()
            .finish()
            .unwrap();
        assert_eq!(r, r2);

        assert!("bytes=".parse::<Range>().is_err())
    }

    #[test]
    fn test_parse_bytes_range_valid() {
        let r: Range = test_decode(&["bytes=1-100"]).unwrap();
        let r2 = ByteRangeBuilder::new()
            .range(1..=100)
            .unwrap()
            .finish()
            .unwrap();
        assert_eq!(r, r2);

        let r: Range = test_decode(&["bytes=1-100,200-"]).unwrap();
        let r2 = Range::Bytes(vec![
            ByteRangeSpec::FromTo(1, 100),
            ByteRangeSpec::AllFrom(200),
        ]);
        assert_eq!(r, r2);

        let r: Range = test_decode(&["bytes=1-100,-100"]).unwrap();
        let r2 = Range::Bytes(vec![
            ByteRangeSpec::FromTo(1, 100),
            ByteRangeSpec::Last(NonZeroU64::new(100).unwrap()),
        ]);
        assert_eq!(r, r2);

        let r: Range = test_decode(&["custom=1-100,-100"]).unwrap();
        let r2 = Range::Unregistered {
            unit: "custom".to_owned(),
            set: "1-100,-100".to_owned(),
        };
        assert_eq!(r, r2);
    }

    #[test]
    fn test_parse_unregistered_range_valid() {
        let r: Range = test_decode(&["custom=1-100,-100"]).unwrap();
        let r2 = Range::Unregistered {
            unit: "custom".to_owned(),
            set: "1-100,-100".to_owned(),
        };
        assert_eq!(r, r2);

        let r: Range = test_decode(&["custom=abcd"]).unwrap();
        let r2 = Range::Unregistered {
            unit: "custom".to_owned(),
            set: "abcd".to_owned(),
        };
        assert_eq!(r, r2);

        let r: Range = test_decode(&["custom=xxx-yyy"]).unwrap();
        let r2 = Range::Unregistered {
            unit: "custom".to_owned(),
            set: "xxx-yyy".to_owned(),
        };
        assert_eq!(r, r2);
    }

    #[test]
    fn test_parse_invalid() {
        let r: Option<Range> = test_decode(&["bytes=1-a,-"]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["bytes=1-2-3"]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["abc"]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["bytes=1-100="]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["bytes="]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["custom="]);
        assert_eq!(r, None);

        let r: Option<Range> = test_decode(&["=1-100"]);
        assert_eq!(r, None);
    }

    #[test]
    fn test_fmt() {
        use crate::HeaderMap as Headers;
        use crate::HeaderMapExt;

        let mut headers = Headers::new();

        headers.typed_insert(Range::Bytes(vec![
            ByteRangeSpec::FromTo(0, 1000),
            ByteRangeSpec::AllFrom(2000),
        ]));
        assert_eq!(headers["Range"], "bytes=0-1000,2000-");

        headers.typed_insert(Range::Unregistered {
            unit: "custom".to_owned(),
            set: "1-xxx".to_owned(),
        });

        assert_eq!(headers["Range"], "custom=1-xxx");
    }

    #[test]
    fn test_byte_range_spec_to_satisfiable_range() {
        assert_eq!(
            (0, Some(0)),
            reduce_bounds(
                ByteRangeSpec::FromTo(0, 0)
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );
        assert_eq!(
            (1, Some(2)),
            reduce_bounds(
                ByteRangeSpec::FromTo(1, 2)
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );
        assert_eq!(
            (1, Some(2)),
            reduce_bounds(
                ByteRangeSpec::FromTo(1, 5)
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );

        assert!(ByteRangeSpec::FromTo(3, 3)
            .to_satisfiable_range_bounds(3)
            .is_err());
        assert!(ByteRangeSpec::FromTo(2, 1)
            .to_satisfiable_range_bounds(3)
            .is_err());
        assert!(ByteRangeSpec::FromTo(0, 0)
            .to_satisfiable_range_bounds(0)
            .is_err());

        assert_eq!(
            (0, Some(2)),
            reduce_bounds(
                ByteRangeSpec::AllFrom(0)
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );
        assert_eq!(
            (2, Some(2)),
            reduce_bounds(
                ByteRangeSpec::AllFrom(2)
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );

        assert!(ByteRangeSpec::AllFrom(3)
            .to_satisfiable_range_bounds(3)
            .is_err());
        assert!(ByteRangeSpec::AllFrom(5)
            .to_satisfiable_range_bounds(3)
            .is_err());
        assert!(ByteRangeSpec::AllFrom(0)
            .to_satisfiable_range_bounds(0)
            .is_err());

        assert_eq!(
            (1, Some(2)),
            reduce_bounds(
                ByteRangeSpec::Last(NonZeroU64::new(2).unwrap())
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );
        assert_eq!(
            (2, Some(2)),
            reduce_bounds(
                ByteRangeSpec::Last(NonZeroU64::new(1).unwrap())
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );
        assert_eq!(
            (0, Some(2)),
            reduce_bounds(
                ByteRangeSpec::Last(NonZeroU64::new(5).unwrap())
                    .to_satisfiable_range_bounds(3)
                    .unwrap()
            )
        );

        assert!(ByteRangeSpec::Last(NonZeroU64::new(2).unwrap())
            .to_satisfiable_range_bounds(0)
            .is_err());
    }
}
