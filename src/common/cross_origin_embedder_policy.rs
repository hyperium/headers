use std::convert::TryFrom;

use headers_core::HeaderName;
use util::{IterExt, TryFromValues};
use Header;
use HeaderValue;

/// Allows a server to declare an embedder policy for a given document.
///
/// The HTTP `Cross-Origin-Embedder-Policy` (COEP) response header prevents a
/// document from loading any cross-origin resources that don't explicitly
/// grant the document permission (using CORP or CORS).
///
/// ## ABNF
///
/// ```text
/// Cross-Origin-Embedder-Policy = "Cross-Origin-Embedder-Policy" ":" unsafe-none | require-corp
/// ```
///
/// ## Possible values
/// * `unsafe-none`
/// * `require-corp`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::CrossOriginEmbedderPolicy;
/// use std::convert::TryFrom;
///
/// let no_corp = CrossOriginEmbedderPolicy::UnsafeNone;
/// let require_corp = CrossOriginEmbedderPolicy::RequireCorp;
/// let coep = CrossOriginEmbedderPolicy::try_from("require-corp");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrossOriginEmbedderPolicy {
    /// `Cross-Origin-Embedder-Policy: require-corp`
    RequireCorp,
    /// `Cross-Origin-Embedder-Policy: unsafe-none`
    UnsafeNone,
}

impl Header for CrossOriginEmbedderPolicy {
    fn name() -> &'static HeaderName {
        &http::header::CROSS_ORIGIN_EMBEDDER_POLICY
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        TryFromValues::try_from_values(values)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.into()));
    }
}

impl TryFrom<&str> for CrossOriginEmbedderPolicy {
    type Error = ::Error;

    fn try_from(s: &str) -> Result<Self, ::Error> {
        let header_value = HeaderValue::from_str(s).map_err(|_| ::Error::invalid())?;
        Self::try_from(&header_value)
    }
}

impl TryFrom<&HeaderValue> for CrossOriginEmbedderPolicy {
    type Error = ::Error;

    fn try_from(header_value: &HeaderValue) -> Result<Self, ::Error> {
        if header_value == "require-corp" {
            Ok(Self::RequireCorp)
        } else if header_value == "unsafe-none" {
            Ok(Self::UnsafeNone)
        } else {
            Err(::Error::invalid())
        }
    }
}

impl TryFromValues for CrossOriginEmbedderPolicy {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .ok_or_else(::Error::invalid)
            .and_then(Self::try_from)
    }
}

impl<'a> From<&'a CrossOriginEmbedderPolicy> for HeaderValue {
    fn from(coep: &'a CrossOriginEmbedderPolicy) -> HeaderValue {
        match coep {
            CrossOriginEmbedderPolicy::RequireCorp => HeaderValue::from_static("require-corp"),
            CrossOriginEmbedderPolicy::UnsafeNone => HeaderValue::from_static("unsafe-none"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn unsafe_none() {
        let unsafe_none = test_decode::<CrossOriginEmbedderPolicy>(&["unsafe-none"]).unwrap();
        assert_eq!(unsafe_none, CrossOriginEmbedderPolicy::UnsafeNone);

        let headers = test_encode(unsafe_none);
        assert_eq!(headers["cross-origin-embedder-policy"], "unsafe-none");
    }

    #[test]
    fn require_corp() {
        let require_corp = test_decode::<CrossOriginEmbedderPolicy>(&["require-corp"]).unwrap();
        assert_eq!(require_corp, CrossOriginEmbedderPolicy::RequireCorp);

        let headers = test_encode(require_corp);
        assert_eq!(headers["cross-origin-embedder-policy"], "require-corp");
    }
}
