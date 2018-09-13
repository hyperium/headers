/// `Access-Control-Expose-Headers` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-expose-headers-response-header)
///
/// The Access-Control-Expose-Headers header indicates which headers are safe to expose to the
/// API of a CORS API specification.
///
/// # ABNF
///
/// ```text
/// Access-Control-Expose-Headers = "Access-Control-Expose-Headers" ":" #field-name
/// ```
///
/// # Example values
/// * `ETag, Content-Length`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// extern crate http;
/// # fn main() {
/// use http::header::{CONTENT_LENGTH, ETAG};
/// use headers::AccessControlExposeHeaders;
///
/// let expose = AccessControlExposeHeaders::new(vec![
///     CONTENT_LENGTH,
///     ETAG,
/// ]);
/// # }
/// ```
#[derive(Clone, Debug, Header)]
#[header(csv)]
pub struct AccessControlExposeHeaders(Vec<::HeaderName>);

impl AccessControlExposeHeaders {
    /// Create an `AccessControlExposeHeaders` from an iterator of header names.
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=::HeaderName>,
    {
        let headers = iter
            .into_iter()
            .collect();

        AccessControlExposeHeaders(headers)
    }
}
