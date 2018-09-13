/// `Access-Control-Request-Headers` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-request-headers-request-header)
///
/// The `Access-Control-Request-Headers` header indicates which headers will
/// be used in the actual request as part of the preflight request.
/// during the actual request.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Headers: "Access-Control-Allow-Headers" ":" #field-name
/// ```
///
/// # Example values
/// * `accept-language, date`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// extern crate http;
/// # fn main() {
/// use http::header::{ACCEPT_LANGUAGE, DATE};
/// use headers::AccessControlRequestHeaders;
///
/// let req_headers = AccessControlRequestHeaders::new(vec![
///     ACCEPT_LANGUAGE,
///     DATE,
/// ]);
/// # }
/// ```
#[derive(Clone, Debug, Header)]
#[header(csv)]
pub struct AccessControlRequestHeaders(Vec<::HeaderName>);

impl AccessControlRequestHeaders {
    /// Create an `AccessControlRequestHeaders` from an iterator of header names.
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=::HeaderName>,
    {
        let headers = iter
            .into_iter()
            .collect();

        AccessControlRequestHeaders(headers)
    }
}
