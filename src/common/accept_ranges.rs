use http::HeaderValue;

use crate::util::FlatCsv;

/// `Accept-Ranges` header, defined in [RFC7233](http://tools.ietf.org/html/rfc7233#section-2.3)
///
/// The `Accept-Ranges` header field allows a server to indicate that it
/// supports range requests for the target resource.
///
/// # ABNF
///
/// ```text
/// Accept-Ranges     = acceptable-ranges
/// acceptable-ranges = 1#range-unit / \"none\"
///
/// # Example values
/// * `bytes`
/// * `none`
/// * `unknown-unit`
/// ```
///
/// # Examples
///
/// ```
/// use headers::{AcceptRanges, HeaderMap, HeaderMapExt};
///
/// let mut headers = HeaderMap::new();
///
/// headers.typed_insert(AcceptRanges::bytes());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptRanges(FlatCsv);

derive_header! {
    AcceptRanges(_),
    name: ACCEPT_RANGES
}

const ACCEPT_RANGES_BYTES: &str = "bytes";

impl AcceptRanges {
    /// A constructor to easily create the common `Accept-Ranges: bytes` header.
    pub fn bytes() -> Self {
        AcceptRanges(HeaderValue::from_static(ACCEPT_RANGES_BYTES).into())
    }

    /// Check if the unit is `bytes`.
    pub fn is_bytes(&self) -> bool {
        self.0.value == ACCEPT_RANGES_BYTES
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::*;

    fn accept_ranges(s: &str) -> AcceptRanges {
        test_decode(&[s]).unwrap()
    }

    #[test]
    fn bytes_constructor() {
        assert_eq!(accept_ranges("bytes"), AcceptRanges::bytes());
    }

    #[test]
    fn is_bytes_method_successful_with_bytes_ranges() {
        assert!(accept_ranges("bytes").is_bytes());
    }

    #[test]
    fn is_bytes_method_successful_with_bytes_ranges_by_constructor() {
        assert!(AcceptRanges::bytes().is_bytes());
    }

    #[test]
    fn is_bytes_method_failed_with_not_bytes_ranges() {
        assert!(!accept_ranges("dummy").is_bytes());
    }
}
