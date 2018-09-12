use ::{Header, HeaderName, HeaderValue};

/// `Access-Control-Allow-Credentials` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
///
/// > The Access-Control-Allow-Credentials HTTP response header indicates whether the
/// > response to request can be exposed when the credentials flag is true. When part
/// > of the response to an preflight request it indicates that the actual request can
/// > be made with credentials. The Access-Control-Allow-Credentials HTTP header must
/// > match the following ABNF:
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Credentials: "Access-Control-Allow-Credentials" ":" "true"
/// ```
///
/// Since there is only one acceptable field value, the header struct does not accept
/// any values at all. Setting an empty `AccessControlAllowCredentials` header is
/// sufficient. See the examples below.
///
/// # Example values
/// * "true"
///
/// # Examples
///
/// ```
/// use headers::AccessControlAllowCredentials;
///
/// let allow_creds = AccessControlAllowCredentials;
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccessControlAllowCredentials;

impl Header for AccessControlAllowCredentials {
    const NAME: &'static HeaderName = &::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS;

    fn decode(values: &mut ::Values) -> ::Result<Self> {
        let value = values.next_or_empty()?;
        if value.as_bytes().eq_ignore_ascii_case(b"true") {
            Ok(AccessControlAllowCredentials)
        } else {
            Err(::Error::invalid())
        }
    }

    fn encode(&self, values: &mut ::ToValues) {
        values.append(HeaderValue::from_static("true"))
    }
}
