use http::HeaderValue;

use crate::util::FlatCsv;

/// `Transfer-Encoding` header, defined in
/// [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-3.3.1)
///
/// The `Transfer-Encoding` header field lists the transfer coding names
/// corresponding to the sequence of transfer codings that have been (or
/// will be) applied to the payload body in order to form the message
/// body.
///
/// Note that setting this header will *remove* any previously set
/// `Content-Length` header, in accordance with
/// [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-3.3.2):
///
/// > A sender MUST NOT send a Content-Length header field in any message
/// > that contains a Transfer-Encoding header field.
///
/// # ABNF
///
/// ```text
/// Transfer-Encoding = 1#transfer-coding
/// ```
///
/// # Example values
///
/// * `chunked`
/// * `gzip, chunked`
///
/// # Example
///
/// ```
/// use headers::TransferEncoding;
///
/// let transfer = TransferEncoding::chunked();
/// ```
// This currently is just a `HeaderValue`, instead of a `Vec<Encoding>`, since
// the most common by far instance is simply the string `chunked`. It'd be a
// waste to need to allocate just for that.
#[derive(Clone, Debug)]
pub struct TransferEncoding(FlatCsv);

derive_header! {
    TransferEncoding(_),
    name: TRANSFER_ENCODING
}

impl TransferEncoding {
    /// Constructor for the most common Transfer-Encoding, `chunked`.
    pub fn chunked() -> TransferEncoding {
        TransferEncoding(HeaderValue::from_static("chunked").into())
    }

    /// Returns whether this ends with the `chunked` encoding.
    pub fn is_chunked(&self) -> bool {
        let value = match self.0.value.to_str() {
            Ok(value) => value.as_bytes(),
            Err(_) => return false,
        };
        let mut encoding = value
            .rsplit(|&byte| byte == b',')
            .next()
            .unwrap_or_default();

        // HeaderValue::to_str permits only SP and HTAB whitespace.
        while let Some((&byte, rest)) = encoding.split_first() {
            if byte != b' ' && byte != b'\t' {
                break;
            }
            encoding = rest;
        }
        while let Some((&byte, rest)) = encoding.split_last() {
            if byte != b' ' && byte != b'\t' {
                break;
            }
            encoding = rest;
        }

        encoding == b"chunked"
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::TransferEncoding;

    #[test]
    fn chunked_is_chunked() {
        assert!(TransferEncoding::chunked().is_chunked());
    }

    #[test]
    fn decode_gzip_chunked_is_chunked() {
        let te = test_decode::<TransferEncoding>(&["gzip, chunked"]).unwrap();
        assert!(te.is_chunked());
    }

    #[test]
    fn decode_chunked_gzip_is_not_chunked() {
        let te = test_decode::<TransferEncoding>(&["chunked, gzip"]).unwrap();
        assert!(!te.is_chunked());
    }

    #[test]
    fn decode_notchunked_is_not_chunked() {
        let te = test_decode::<TransferEncoding>(&["notchunked"]).unwrap();
        assert!(!te.is_chunked());
    }

    #[test]
    fn decode_multiple_is_chunked() {
        let te = test_decode::<TransferEncoding>(&["gzip", "chunked"]).unwrap();
        assert!(te.is_chunked());
    }
}
