use std::convert::TryInto;
use {Header, HeaderValue};

/// `Content-MD5` header, defined in
/// [RFC1864](https://datatracker.ietf.org/doc/html/rfc1864)
///
/// ## ABNF
///
/// ```text
/// Content-Length = 1*DIGIT
/// ```
///
/// ## Example values
///
/// * `Q2hlY2sgSW50ZWdyaXR5IQ==`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// # extern crate http;
/// # extern crate headers_core;
/// use http::HeaderValue;
/// use headers::ContentMd5;
/// use headers_core::Header;
///
/// let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
///
/// let md5 = ContentMd5::decode(&mut [value].into_iter()).unwrap();
/// assert_eq!(md5.0, "Check Integrity!".as_bytes())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContentMd5(pub [u8; 16]);

static CONTENT_MD5: ::http::header::HeaderName =
    ::http::header::HeaderName::from_static("content-md5");

impl Header for ContentMd5 {
    fn name() -> &'static ::http::header::HeaderName {
        &CONTENT_MD5
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        let value = values.next().ok_or_else(::Error::invalid)?;

        // Ensure base64 encoded length fits the expected MD5 digest length.
        if value.len() < 22 || value.len() > 24 {
            return Err(::Error::invalid());
        }

        let value = value.to_str().map_err(|_| ::Error::invalid())?;
        let vec = base64::decode(value).map_err(|_| ::Error::invalid())?;
        Ok(Self(vec[..16].try_into().map_err(|_| ::Error::invalid())?))
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        let encoded = base64::encode(self.0);
        if let Ok(value) = HeaderValue::from_str(&encoded) {
            values.extend(std::iter::once(value));
        }
    }
}
