use bytes::Bytes;
use headers_core::decode::TryFromValues;
use http::uri::{self, Authority, Scheme, Uri};
use std::fmt;
use ::{HeaderValue};

/// The `Origin` header.
///
/// The `Origin` header is a version of the `Referer` header that is used for all HTTP fetches and `POST`s whose CORS flag is set.
/// This header is often used to inform recipients of the security context of where the request was initiated.
///
/// Following the spec, [https://fetch.spec.whatwg.org/#origin-header][url], the value of this header is composed of
/// a String (scheme), Host (host/port)
///
/// [url]: https://fetch.spec.whatwg.org/#origin-header
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::Origin;
///
/// let origin = Origin::NULL;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Header)]
pub struct Origin(OriginOrNull);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OriginOrNull {
    Origin(Scheme, Authority),
    Null,
}

impl Origin {
    /// The literal `null` Origin header.
    pub const NULL: Origin = Origin(OriginOrNull::Null);

    /// Checks if `Origin` is `null`.
    pub fn is_null(&self) -> bool {
        match self.0 {
            OriginOrNull::Null => true,
            _ => false,
        }
    }

    /// Get the "scheme" part of this origin.
    pub fn scheme(&self) -> &str {
        match self.0 {
            OriginOrNull::Origin(ref scheme, _) => scheme.as_str(),
            OriginOrNull::Null => "",
        }
    }

    /// Get the "hostname" part of this origin.
    pub fn hostname(&self) -> &str {
        match self.0 {
            OriginOrNull::Origin(_, ref auth) => auth.host(),
            OriginOrNull::Null => "",
        }
    }

    /// Get the "port" part of this origin.
    pub fn port(&self) -> Option<u16> {
        match self.0 {
            OriginOrNull::Origin(_, ref auth) => auth.port(),
            OriginOrNull::Null => None,
        }
    }

    // Used in AccessControlAllowOrigin
    pub(super) fn try_from_value(value: &HeaderValue) -> Option<Self> {
        OriginOrNull::try_from_value(value)
            .map(Origin)
    }

    pub(super) fn into_value(&self) -> HeaderValue {
        (&self.0).into()
    }
}

impl OriginOrNull {
    fn try_from_value(value: &HeaderValue) -> Option<Self> {
        if value == "null" {
            return Some(OriginOrNull::Null);
        }

        let bytes = Bytes::from(value.clone());

        let uri = Uri::from_shared(bytes).ok()?;

        let (scheme, auth) = match uri.into_parts() {
            uri::Parts {
                scheme: Some(scheme),
                authority: Some(auth),
                path_and_query: None,
                ..
            } => (scheme, auth),
            _ => {
                return None;
            }
        };

        Some(OriginOrNull::Origin(scheme, auth))
    }
}

impl TryFromValues for OriginOrNull {
    fn try_from_values(values: &mut ::Values) -> Option<OriginOrNull> {
        values
            .next()
            .and_then(OriginOrNull::try_from_value)
    }
}

impl<'a> From<&'a OriginOrNull> for HeaderValue {
    fn from(origin: &'a OriginOrNull) -> HeaderValue {
        match origin {
            OriginOrNull::Origin(ref scheme, ref auth) => {
                let s = format!("{}://{}", scheme, auth);
                let bytes = Bytes::from(s);
                HeaderValue::from_shared(bytes)
                    .expect("Scheme and Authority are valid header values")
            },
            // Serialized as "null" per ASCII serialization of an origin
            // https://html.spec.whatwg.org/multipage/browsers.html#ascii-serialisation-of-an-origin
            OriginOrNull::Null => HeaderValue::from_static("null"),
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            OriginOrNull::Origin(ref scheme, ref auth) => {
                f.write_str(&format!("{}://{}", scheme, auth))
            },
            OriginOrNull::Null => f.write_str("null"),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::Origin;
    use Header;
    use std::borrow::Cow;

    #[test]
    fn test_origin() {
        let origin : Origin = Header::parse_header(&vec![b"http://foo.com".to_vec()].into()).unwrap();
        assert_eq!(&origin, &Origin::new("http", "foo.com", None));
        assert_borrowed!(origin.scheme().unwrap().into());

        let origin : Origin = Header::parse_header(&vec![b"https://foo.com:443".to_vec()].into()).unwrap();
        assert_eq!(&origin, &Origin::new("https", "foo.com", Some(443)));
        assert_borrowed!(origin.scheme().unwrap().into());
    }
}
*/
