use http::header::HeaderName;

/// `Access-Control-Allow-Headers` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
///
/// The `Access-Control-Allow-Headers` header indicates, as part of the
/// response to a preflight request, which header field names can be used
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
/// use http::header::DATE;
/// use headers::AccessControlAllowHeaders;
///
/// let allow_headers = AccessControlAllowHeaders::new(vec![
///     DATE,
/// ]);
/// ```
#[derive(Clone, Debug, PartialEq, Header)]
#[header(csv)]
pub struct AccessControlAllowHeaders(Vec<HeaderName>);

impl AccessControlAllowHeaders {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=HeaderName>,
    {
        let headers = iter
            .into_iter()
            .collect();

        AccessControlAllowHeaders(headers)
    }
}
