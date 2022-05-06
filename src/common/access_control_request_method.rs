use crate::core::{Decodable, Encodable, Named};
use crate::{HeaderName, HeaderValue};

use http::Method;

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
/// # extern crate headers;
/// extern crate http;
/// use headers::AccessControlRequestMethod;
/// use http::Method;
///
/// let req_method = AccessControlRequestMethod::from(Method::GET);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControlRequestMethod(Method);

impl Named for AccessControlRequestMethod {
    fn name() -> &'static HeaderName {
        &::http::header::ACCESS_CONTROL_REQUEST_METHOD
    }
}

impl Decodable for AccessControlRequestMethod {
    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|value| Method::from_bytes(value.as_bytes()).ok())
            .map(AccessControlRequestMethod)
            .ok_or_else(::Error::invalid)
    }
}

impl Encodable for AccessControlRequestMethod {
    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        // For the more common methods, try to use a static string.
        let s = match self.0 {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            _ => {
                let val = HeaderValue::from_str(self.0.as_ref())
                    .expect("Methods are also valid HeaderValues");
                values.extend(::std::iter::once(val));
                return;
            }
        };

        values.extend(::std::iter::once(HeaderValue::from_static(s)));
    }
}

impl From<Method> for AccessControlRequestMethod {
    fn from(method: Method) -> AccessControlRequestMethod {
        AccessControlRequestMethod(method)
    }
}

impl From<AccessControlRequestMethod> for Method {
    fn from(method: AccessControlRequestMethod) -> Method {
        method.0
    }
}
