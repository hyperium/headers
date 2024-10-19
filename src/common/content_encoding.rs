use http::HeaderValue;

use self::sealed::AsCoding;
use crate::util::FlatCsv;

/// `Content-Encoding` header, defined in
/// [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-3.1.2.2)
///
/// The `Content-Encoding` header field indicates what content codings
/// have been applied to the representation, beyond those inherent in the
/// media type, and thus what decoding mechanisms have to be applied in
/// order to obtain data in the media type referenced by the Content-Type
/// header field.  Content-Encoding is primarily used to allow a
/// representation's data to be compressed without losing the identity of
/// its underlying media type.
///
/// # ABNF
///
/// ```text
/// Content-Encoding = 1#content-coding
/// ```
///
/// # Example values
///
/// * `gzip`
/// * `br`
/// * `zstd`
///
/// # Examples
///
/// ```
/// use headers::ContentEncoding;
///
/// let content_enc = ContentEncoding::gzip();
/// ```
#[derive(Clone, Debug)]
pub struct ContentEncoding(FlatCsv);

derive_header! {
    ContentEncoding(_),
    name: CONTENT_ENCODING
}

impl ContentEncoding {
    /// A constructor to easily create a `Content-Encoding: gzip` header.
    #[inline]
    pub fn gzip() -> ContentEncoding {
        ContentEncoding(HeaderValue::from_static("gzip").into())
    }

    /// A constructor to easily create a `Content-Encoding: br` header.
    #[inline]
    pub fn brotli() -> ContentEncoding {
        ContentEncoding(HeaderValue::from_static("br").into())
    }

    /// A constructor to easily create a `Content-Encoding: zstd` header.
    #[inline]
    pub fn zstd() -> ContentEncoding {
        ContentEncoding(HeaderValue::from_static("zstd").into())
    }

    /// Check if this header contains a given "coding".
    ///
    /// This can be used with these argument types:
    ///
    /// - `&str`
    ///
    /// # Example
    ///
    /// ```
    /// use headers::ContentEncoding;
    ///
    /// let content_enc = ContentEncoding::gzip();
    ///
    /// assert!(content_enc.contains("gzip"));
    /// assert!(!content_enc.contains("br"));
    /// ```
    pub fn contains(&self, coding: impl AsCoding) -> bool {
        let s = coding.as_coding();
        self.0.iter().any(|opt| opt == s)
    }
}

mod sealed {
    pub trait AsCoding: Sealed {}

    pub trait Sealed {
        fn as_coding(&self) -> &str;
    }

    impl AsCoding for &str {}

    impl Sealed for &str {
        fn as_coding(&self) -> &str {
            self
        }
    }
}
