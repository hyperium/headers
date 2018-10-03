use http::Method;
use ::{Header, HeaderName, HeaderValue};

/// `Access-Control-Request-Method` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-request-method-request-header)
///
/// The `Access-Control-Request-Method` header indicates which method will be
/// used in the actual request as part of the preflight request.
/// # ABNF
///
/// ```text
/// Access-Control-Request-Method: \"Access-Control-Request-Method\" \":\" Method
/// ```
///
/// # Example values
/// * `GET`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// extern crate http;
/// use headers::AccessControlRequestMethod;
/// use http::Method;
///
/// let req_method = AccessControlRequestMethod::from(Method::GET);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControlRequestMethod(Method);

impl Header for AccessControlRequestMethod {
    const NAME: &'static HeaderName = &::http::header::ACCESS_CONTROL_REQUEST_METHOD;

    fn decode(values: &mut ::Values) -> Option<Self> {
        values.next()
            .and_then(|value| {
                Method::from_bytes(value.as_bytes()).ok()
            })
            .map(AccessControlRequestMethod)
    }

    fn encode(&self, values: &mut ::ToValues) {
        // For the more common methods, try to use a static string.
        let s = match self.0 {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            _ => {
                let val = HeaderValue::from_str(self.0.as_ref())
                    .expect("Methods are also valid HeaderValues");
                values.append(val);
                return;
            }
        };

        values.append(HeaderValue::from_static(s))
    }
}

impl From<Method> for AccessControlRequestMethod {
    fn from(method: Method) -> AccessControlRequestMethod {
        AccessControlRequestMethod(method)
    }
}

