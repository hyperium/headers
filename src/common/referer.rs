use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use bytes::Bytes;
use http::uri::{Authority, PathAndQuery, Scheme, Uri};
use http::HeaderValue;

use crate::util::{HeaderValueString, IterExt, TryFromValues};
use crate::Error;

/// `Referer` header, defined in
/// [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.5.2)
///
/// The `Referer` \[sic\] header field allows the user agent to specify a
/// URI reference for the resource from which the target URI was obtained
/// (i.e., the "referrer", though the field name is misspelled).  A user
/// agent MUST NOT include the fragment and userinfo components of the
/// URI reference, if any, when generating the Referer field value.
///
/// ## ABNF
///
/// ```text
/// Referer = absolute-URI / partial-URI
/// ```
///
/// ## Example values
///
/// * `http://www.example.org/hypertext/Overview.html`
/// * `/People.html`
///
/// # Examples
///
/// ```
/// use headers::Referer;
/// use std::str::FromStr;
///
/// let r = Referer::from_str("http://www.example.org/hypertext/Overview.html").unwrap();
/// assert_eq!(r.scheme(), Some("http"));
/// assert_eq!(r.hostname(), Some("www.example.org"));
/// assert_eq!(r.path(), "/hypertext/Overview.html");
///
/// // Partial URIs work too
/// let r2 = Referer::from_str("/People.html").unwrap();
/// assert_eq!(r2.scheme(), None);
/// assert_eq!(r2.path(), "/People.html");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Referer(RefererUri);

derive_header! {
    Referer(_),
    name: REFERER
}

#[derive(Debug, Clone, PartialEq)]
enum RefererUri {
    /// Absolute URI with scheme and authority
    Absolute {
        scheme: Scheme,
        authority: Authority,
        path_and_query: Option<PathAndQuery>,
    },
    /// Partial URI (relative reference)
    Partial(HeaderValueString),
}

impl Referer {
    /// Create a `Referer` with a static string.
    ///
    /// # Panic
    ///
    /// Panics if the string is not a legal header value or contains
    /// forbidden components (fragment or userinfo).
    pub const fn from_static(s: &'static str) -> Referer {
        Referer(RefererUri::Partial(HeaderValueString::from_static(s)))
    }

    /// Tries to build a `Referer` from components for absolute URIs.
    ///
    /// This method constructs an absolute URI referer from scheme, host,
    /// optional port, and optional path with query.
    pub fn try_from_parts(
        scheme: &str,
        host: &str,
        port: impl Into<Option<u16>>,
        path_and_query: Option<&str>,
    ) -> Result<Self, InvalidReferer> {
        struct MaybePort(Option<u16>);

        impl fmt::Display for MaybePort {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(port) = self.0 {
                    write!(f, ":{}", port)
                } else {
                    Ok(())
                }
            }
        }

        let path_part = path_and_query.unwrap_or("");
        let uri_string = format!("{}://{}{}{}", scheme, host, MaybePort(port.into()), path_part);
        let bytes = Bytes::from(uri_string);
        
        HeaderValue::from_maybe_shared(bytes)
            .ok()
            .and_then(|val| Self::try_from_value(&val))
            .ok_or(InvalidReferer { _inner: () })
    }

    /// Get the "scheme" part of this referer, if it's an absolute URI.
    #[inline]
    pub fn scheme(&self) -> Option<&str> {
        match &self.0 {
            RefererUri::Absolute { scheme, .. } => Some(scheme.as_str()),
            RefererUri::Partial(_) => None,
        }
    }

    /// Get the "hostname" part of this referer, if it's an absolute URI.
    #[inline]
    pub fn hostname(&self) -> Option<&str> {
        match &self.0 {
            RefererUri::Absolute { authority, .. } => Some(authority.host()),
            RefererUri::Partial(_) => None,
        }
    }

    /// Get the "port" part of this referer, if it's an absolute URI.
    #[inline]
    pub fn port(&self) -> Option<u16> {
        match &self.0 {
            RefererUri::Absolute { authority, .. } => authority.port_u16(),
            RefererUri::Partial(_) => None,
        }
    }

    /// Get the "path" part of this referer.
    ///
    /// For absolute URIs, this extracts the path component.
    /// For partial URIs, this returns the entire value if it starts with '/'.
    #[inline]
    pub fn path(&self) -> &str {
        match &self.0 {
            RefererUri::Absolute { path_and_query: Some(pq), .. } => pq.path(),
            RefererUri::Absolute { path_and_query: None, .. } => "/",
            RefererUri::Partial(s) => {
                let s_str = s.as_str();
                if s_str.starts_with('/') {
                    // Extract just the path part if it contains query
                    if let Some(pos) = s_str.find('?') {
                        &s_str[..pos]
                    } else {
                        s_str
                    }
                } else {
                    ""
                }
            }
        }
    }

    /// Get the "query" part of this referer, if present.
    #[inline]
    pub fn query(&self) -> Option<&str> {
        match &self.0 {
            RefererUri::Absolute { path_and_query: Some(pq), .. } => pq.query(),
            RefererUri::Absolute { path_and_query: None, .. } => None,
            RefererUri::Partial(s) => {
                let s_str = s.as_str();
                if let Some(pos) = s_str.find('?') {
                    Some(&s_str[pos + 1..])
                } else {
                    None
                }
            }
        }
    }

    /// Returns true if this is an absolute URI (has scheme and authority).
    #[inline]
    pub fn is_absolute(&self) -> bool {
        matches!(self.0, RefererUri::Absolute { .. })
    }

    /// Returns true if this is a partial URI (relative reference).
    #[inline]
    pub fn is_partial(&self) -> bool {
        matches!(self.0, RefererUri::Partial(_))
    }

    // Used internally and by other modules
    pub(super) fn try_from_value(value: &HeaderValue) -> Option<Self> {
        RefererUri::try_from_value(value).map(Referer)
    }
}

error_type!(InvalidReferer);

impl RefererUri {
    fn try_from_value(value: &HeaderValue) -> Option<Self> {
        let value_str = value.to_str().ok()?;
        
        // Check for forbidden components
        if value_str.contains('#') {
            // Contains fragment, which is forbidden
            return None;
        }
        
        if value_str.contains('@') {
            // Might contain userinfo, which is forbidden
            // This is a simple check; a more thorough check would parse the URI
            if let Ok(uri) = Uri::try_from(value_str) {
                if uri.authority().map_or(false, |auth| auth.as_str().contains('@')) {
                    return None;
                }
            }
        }

        // Try to parse as URI first
        if let Ok(uri) = Uri::try_from(value_str) {
            let parts = uri.into_parts();
            
            // If it has scheme and authority, it's an absolute URI
            if let (Some(scheme), Some(authority)) = (parts.scheme, parts.authority) {
                return Some(RefererUri::Absolute {
                    scheme,
                    authority,
                    path_and_query: parts.path_and_query,
                });
            }
        }

        // Otherwise, treat as partial URI
        HeaderValueString::from_str(value_str)
            .map(RefererUri::Partial)
            .ok()
    }
}

impl TryFromValues for RefererUri {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(RefererUri::try_from_value)
            .ok_or_else(Error::invalid)
    }
}

impl FromStr for Referer {
    type Err = InvalidReferer;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // Create a temporary HeaderValue to reuse our parsing logic
        HeaderValue::from_str(src)
            .ok()
            .and_then(|val| Self::try_from_value(&val))
            .ok_or(InvalidReferer { _inner: () })
    }
}

impl fmt::Display for Referer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            RefererUri::Absolute { scheme, authority, path_and_query } => {
                write!(f, "{}://{}", scheme, authority)?;
                if let Some(pq) = path_and_query {
                    write!(f, "{}", pq)
                } else {
                    Ok(())
                }
            }
            RefererUri::Partial(s) => fmt::Display::fmt(s, f),
        }
    }
}

impl<'a> From<&'a RefererUri> for HeaderValue {
    fn from(referer: &'a RefererUri) -> HeaderValue {
        match referer {
            RefererUri::Absolute { scheme, authority, path_and_query } => {
                let mut s = format!("{}://{}", scheme, authority);
                if let Some(pq) = path_and_query {
                    s.push_str(pq.as_str());
                }
                let bytes = Bytes::from(s);
                HeaderValue::from_maybe_shared(bytes)
                    .expect("Scheme, Authority, and PathAndQuery are valid header values")
            }
            RefererUri::Partial(s) => s.as_str().parse()
                .expect("HeaderValueString contains valid header value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn absolute_referer() {
        let s = "http://www.example.org/hypertext/Overview.html";
        let referer = test_decode::<Referer>(&[s]).unwrap();
        assert_eq!(referer.scheme(), Some("http"));
        assert_eq!(referer.hostname(), Some("www.example.org"));
        assert_eq!(referer.port(), None);
        assert_eq!(referer.path(), "/hypertext/Overview.html");
        assert_eq!(referer.query(), None);
        assert!(referer.is_absolute());
        assert!(!referer.is_partial());

        let headers = test_encode(referer);
        assert_eq!(headers["referer"], s);
    }

    #[test]
    fn absolute_referer_with_port_and_query() {
        let s = "https://example.com:8443/api/users?page=1";
        let referer = test_decode::<Referer>(&[s]).unwrap();
        assert_eq!(referer.scheme(), Some("https"));
        assert_eq!(referer.hostname(), Some("example.com"));
        assert_eq!(referer.port(), Some(8443));
        assert_eq!(referer.path(), "/api/users");
        assert_eq!(referer.query(), Some("page=1"));
        assert!(referer.is_absolute());

        let headers = test_encode(referer);
        assert_eq!(headers["referer"], s);
    }

    #[test]
    fn partial_referer() {
        let s = "/People.html";
        let referer = test_decode::<Referer>(&[s]).unwrap();
        assert_eq!(referer.scheme(), None);
        assert_eq!(referer.hostname(), None);
        assert_eq!(referer.port(), None);
        assert_eq!(referer.path(), "/People.html");
        assert_eq!(referer.query(), None);
        assert!(!referer.is_absolute());
        assert!(referer.is_partial());

        let headers = test_encode(referer);
        assert_eq!(headers["referer"], s);
    }

    #[test]
    fn partial_referer_with_query() {
        let s = "/search?q=rust";
        let referer = test_decode::<Referer>(&[s]).unwrap();
        assert_eq!(referer.path(), "/search");
        assert_eq!(referer.query(), Some("q=rust"));
        assert!(referer.is_partial());
    }

    #[test]
    fn try_from_parts() {
        let referer = Referer::try_from_parts("https", "example.com", Some(443), Some("/api/test?v=1")).unwrap();
        assert_eq!(referer.scheme(), Some("https"));
        assert_eq!(referer.hostname(), Some("example.com"));
        assert_eq!(referer.port(), Some(443));
        assert_eq!(referer.path(), "/api/test");
        assert_eq!(referer.query(), Some("v=1"));
    }

    #[test]
    fn invalid_referer_with_fragment() {
        // Should reject URIs with fragments
        assert!(test_decode::<Referer>(&["http://example.com/page#section"]).is_none());
    }

    #[test]
    fn invalid_referer_with_userinfo() {
        // Should reject URIs with userinfo
        assert!(test_decode::<Referer>(&["http://user:pass@example.com/page"]).is_none());
    }
}
