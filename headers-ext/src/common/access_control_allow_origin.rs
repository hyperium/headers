use headers_core::decode::TryFromValues;
use ::{HeaderValue};
use super::origin::{Origin};

/// The `Access-Control-Allow-Origin` response header,
/// part of [CORS](http://www.w3.org/TR/cors/#access-control-allow-origin-response-header)
///
/// The `Access-Control-Allow-Origin` header indicates whether a resource
/// can be shared based by returning the value of the Origin request header,
/// `*`, or `null` in the response.
///
/// ## ABNF
///
/// ```text
/// Access-Control-Allow-Origin = "Access-Control-Allow-Origin" ":" origin-list-or-null | "*"
/// ```
///
/// ## Example values
/// * `null`
/// * `*`
/// * `http://google.com/`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::AccessControlAllowOrigin;
///
/// let any_origin = AccessControlAllowOrigin::ANY;
/// let null_origin = AccessControlAllowOrigin::NULL;
/// # //let allow_origin = AccessControlAllowOrigin::
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Header)]
pub struct AccessControlAllowOrigin(OriginOrAny);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OriginOrAny {
    Origin(Origin),
    /// Allow all origins
    Any,
}

impl AccessControlAllowOrigin {
    pub const ANY: AccessControlAllowOrigin = AccessControlAllowOrigin(OriginOrAny::Any);
    pub const NULL: AccessControlAllowOrigin = AccessControlAllowOrigin(OriginOrAny::Origin(Origin::NULL));
}

impl TryFromValues for OriginOrAny {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        let value = values.next()?;

        if value == "*" {
            return Some(OriginOrAny::Any);
        }

        Origin::try_from_value(value)
            .map(OriginOrAny::Origin)
    }
}

impl<'a> From<&'a OriginOrAny> for HeaderValue {
    fn from(origin: &'a OriginOrAny) -> HeaderValue {
        match origin {
            OriginOrAny::Origin(ref origin) => origin.into_value(),
            OriginOrAny::Any => HeaderValue::from_static("*"),
        }
    }
}

