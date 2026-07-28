use http::HeaderValue;

use crate::Error;

//pub use self::charset::Charset;
//pub use self::encoding::Encoding;
pub(crate) use self::entity::{EntityTag, EntityTagRange};
pub(crate) use self::flat_csv::{FlatCsv, SemiColon};
pub(crate) use self::fmt::fmt;
pub(crate) use self::http_date::HttpDate;
pub(crate) use self::iter::IterExt;
//pub use language_tags::LanguageTag;
//pub use self::quality_value::{Quality, QualityValue};
pub(crate) use self::seconds::Seconds;
pub(crate) use self::value_string::HeaderValueString;

//mod charset;

//mod encoding;
mod entity;
mod flat_csv;
mod fmt;
mod http_date;
mod iter;
//mod quality_value;
mod seconds;
mod value_string;

macro_rules! error_type {
    ($name:ident) => {
        #[doc(hidden)]
        pub struct $name {
            _inner: (),
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl ::std::error::Error for $name {}
    };
}

macro_rules! derive_header {
    ($type:ident(_), name: $name:ident) => {
        impl crate::Header for $type {
            fn name() -> &'static ::http::header::HeaderName {
                &::http::header::$name
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, crate::Error>
            where
                I: Iterator<Item = &'i ::http::header::HeaderValue>,
            {
                crate::util::TryFromValues::try_from_values(values).map($type)
            }

            fn encode<E: Extend<crate::HeaderValue>>(&self, values: &mut E) {
                values.extend(::std::iter::once((&self.0).into()));
            }
        }
    };
}

/// A helper trait for use when deriving `Header`.
pub(crate) trait TryFromValues: Sized {
    /// Try to convert from the values into an instance of `Self`.
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>;
}

impl TryFromValues for HeaderValue {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values.next().cloned().ok_or_else(Error::invalid)
    }
}

/// Parse an optional `+` followed by ASCII digits.
#[inline]
pub(crate) fn parse_u64_digits(bytes: &[u8]) -> Option<u64> {
    let digits = match bytes.split_first() {
        Some((b'+', rest)) => rest,
        _ => bytes,
    };
    if digits.is_empty() {
        return None;
    }

    // Values shorter than 20 digits cannot overflow u64.
    if digits.len() < 20 {
        let mut acc: u64 = 0;
        for &b in digits {
            let digit = b.wrapping_sub(b'0');
            if digit > 9 {
                return None;
            }
            acc = acc * 10 + digit as u64;
        }
        return Some(acc);
    }

    let mut acc: u64 = 0;
    for &b in digits {
        let digit = b.wrapping_sub(b'0');
        if digit > 9 {
            return None;
        }
        acc = acc.checked_mul(10)?.checked_add(digit as u64)?;
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::parse_u64_digits;

    #[test]
    fn parse_u64_digits_matches_from_str_edges() {
        for input in [
            "",
            "+",
            "0",
            "+1",
            "9999999999999999999",
            "18446744073709551615",
            "18446744073709551616",
            "00000000000000000000000000000000000000001",
            "-1",
            "1_000",
            "12x",
        ] {
            assert_eq!(
                parse_u64_digits(input.as_bytes()),
                input.parse::<u64>().ok(),
                "mismatch for {:?}",
                input
            );
        }
    }
}
