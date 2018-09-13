use http::Method;

/// `Access-Control-Allow-Methods` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)
///
/// The `Access-Control-Allow-Methods` header indicates, as part of the
/// response to a preflight request, which methods can be used during the
/// actual request.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Methods: "Access-Control-Allow-Methods" ":" #Method
/// ```
///
/// # Example values
/// * `PUT, DELETE, XMODIFY`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// extern crate http;
/// use http::Method;
/// use headers::AccessControlAllowMethods;
///
/// let allow_methods = AccessControlAllowMethods::new(vec![
///     Method::GET,
///     Method::PUT,
/// ]);
/// ```
#[derive(Clone, Debug, PartialEq, Header)]
#[header(csv)]
pub struct AccessControlAllowMethods(Vec<Method>);

impl AccessControlAllowMethods {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=Method>,
    {
        let methods = iter
            .into_iter()
            .collect();

        AccessControlAllowMethods(methods)
    }
}
