use std::convert::TryFrom;

use headers_core::HeaderName;
use util::{IterExt, TryFromValues};
use Header;
use HeaderValue;

/// Prevents other domains from opening/controlling a window.
///
/// The HTTP `Cross-Origin-Opener-Policy` (COOP) response header allows you to
/// ensure a top-level document does not share a browsing context group with
/// cross-origin documents.
///
/// COOP will process-isolate your document and potential attackers can't
/// access your global object if they were to open it in a popup, preventing a
/// set of cross-origin attacks dubbed XS-Leaks.
///
/// If a cross-origin document with COOP is opened in a new window, the opening
/// document will not have a reference to it, and the `window.opener` property
/// of the new window will be `null`. This allows you to have more control over
/// references to a window than `rel=noopener`, which only affects outgoing
/// navigations.
///
/// ## ABNF
///
/// ```text
/// Cross-Origin-Opener-Policy = "Cross-Origin-Opener-Policy" ":" unsafe-none | same-origin-allow-popups | same-origin
/// ```
///
/// ## Possible values
/// * `unsafe-none`
/// * `same-origin-allow-popups`
/// * `same-origin`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::CrossOriginOpenerPolicy;
/// use std::convert::TryFrom;
///
/// let no_corp = CrossOriginOpenerPolicy::UnsafeNone;
/// let same_origin_allow_popups = CrossOriginOpenerPolicy::SameOriginAllowPopups;
/// let same_origin = CrossOriginOpenerPolicy::SameOrigin;
/// let coop = CrossOriginOpenerPolicy::try_from("same-origin");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrossOriginOpenerPolicy {
    /// `Cross-Origin-Opener-Policy: same-origin`
    SameOrigin,
    /// `Cross-Origin-Opener-Policy: same-origin-allow-popups`
    SameOriginAllowPopups,
    /// `Cross-Origin-Opener-Policy: unsafe-none`
    UnsafeNone,
}

impl Header for CrossOriginOpenerPolicy {
    fn name() -> &'static HeaderName {
        &http::header::CROSS_ORIGIN_OPENER_POLICY
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

impl TryFrom<&str> for CrossOriginOpenerPolicy {
    type Error = ::Error;

    fn try_from(s: &str) -> Result<Self, ::Error> {
        let header_value = HeaderValue::from_str(s).map_err(|_| ::Error::invalid())?;
        Self::try_from(&header_value)
    }
}

impl TryFrom<&HeaderValue> for CrossOriginOpenerPolicy {
    type Error = ::Error;

    fn try_from(header_value: &HeaderValue) -> Result<Self, ::Error> {
        if header_value == "same-origin" {
            Ok(Self::SameOrigin)
        } else if header_value == "same-origin-allow-popups" {
            Ok(Self::SameOriginAllowPopups)
        } else if header_value == "unsafe-none" {
            Ok(Self::UnsafeNone)
        } else {
            Err(::Error::invalid())
        }
    }
}

impl TryFromValues for CrossOriginOpenerPolicy {
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

impl<'a> From<&'a CrossOriginOpenerPolicy> for HeaderValue {
    fn from(coop: &'a CrossOriginOpenerPolicy) -> HeaderValue {
        match coop {
            CrossOriginOpenerPolicy::SameOrigin => HeaderValue::from_static("same-origin"),
            CrossOriginOpenerPolicy::SameOriginAllowPopups => HeaderValue::from_static("same-origin-allow-popups"),
            CrossOriginOpenerPolicy::UnsafeNone => HeaderValue::from_static("unsafe-none"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn unsafe_none() {
        let unsafe_none = test_decode::<CrossOriginOpenerPolicy>(&["unsafe-none"]).unwrap();
        assert_eq!(unsafe_none, CrossOriginOpenerPolicy::UnsafeNone);

        let headers = test_encode(unsafe_none);
        assert_eq!(headers["Cross-Origin-Opener-Policy"], "unsafe-none");
    }

    #[test]
    fn same_origin_allow_popups() {
        let same_origin_allow_popups = test_decode::<CrossOriginOpenerPolicy>(&["same-origin-allow-popups"]).unwrap();
        assert_eq!(same_origin_allow_popups, CrossOriginOpenerPolicy::SameOriginAllowPopups);

        let headers = test_encode(same_origin_allow_popups);
        assert_eq!(headers["Cross-Origin-Opener-Policy"], "same-origin-allow-popups");
    }

    #[test]
    fn same_origin() {
        let same_origin = test_decode::<CrossOriginOpenerPolicy>(&["same-origin"]).unwrap();
        assert_eq!(same_origin, CrossOriginOpenerPolicy::SameOrigin);

        let headers = test_encode(same_origin);
        assert_eq!(headers["Cross-Origin-Opener-Policy"], "same-origin");
    }
}
