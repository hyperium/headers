use std::iter::FromIterator;

use bytes::BytesMut;
use http::HeaderValue;

use crate::util::FlatCsv;

/// `Accept-Encoding` header, defined in
/// [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.4)
///
/// The `Accept-Encoding` header field can be used by user agents to
/// indicate what response content-codings are
/// acceptable in the response.  An  `identity` token is used as a synonym
/// for "no encoding" in order to communicate when no encoding is
/// preferred.
///
/// # ABNF
///
/// ```text
/// Accept-Encoding  = #( codings [ weight ] )
/// codings          = content-coding / "identity" / "*"
/// ```
///
/// # Example values
/// * `compress, gzip`
/// * ``
/// * `*`
/// * `compress;q=0.5, gzip;q=1`
/// * `gzip;q=1.0, identity; q=0.5, *;q=0`
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptEncoding(FlatCsv);

derive_header! {
    AcceptEncoding(_),
    name: ACCEPT_ENCODING
}

impl AcceptEncoding {
    /// Iterator the codings with weight.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter()
    }

    /// Create from a iterator of given codings with optional weigth.
    pub fn from_pairs<'a>(pairs: impl Iterator<Item = (&'a str, Option<f32>)>) -> Self {
        let iter = pairs.into_iter().filter_map(|(coding, q)| {
            if let Some(q) = q {
                let mut buf = BytesMut::new();
                buf.extend_from_slice(coding.as_bytes());
                buf.extend_from_slice(&[b';']);
                buf.extend_from_slice(format!("{:.1}", q).as_bytes());
                HeaderValue::from_maybe_shared(buf.freeze()).ok()
            } else {
                HeaderValue::from_str(coding).ok()
            }
        });
        let csv: FlatCsv = FlatCsv::from_iter(iter);

        AcceptEncoding(csv)
    }
}
