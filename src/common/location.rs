use crate::{Error, Header, HeaderName, HeaderValue};
use http::{header, Uri};
use std::iter;

/// `Location` header, defined in
/// [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.2)
///
/// The `Location` header field is used in some responses to refer to a
/// specific resource in relation to the response.  The type of
/// relationship is defined by the combination of request method and
/// status code semantics.
///
/// # ABNF
///
/// ```text
/// Location = URI-reference
/// ```
///
/// # Example values
/// * `/People.html#tim`
/// * `http://www.example.net/index.html`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// # extern crate http;
/// use headers::Location;
/// use http::Uri;
/// let loc = Location::from(Uri::from_static("/auth/login"));
/// ```

#[derive(Clone, Debug, PartialEq)]
pub struct Location(Uri);

impl Location {
    /// Get the uri for this header
    pub fn uri(&self) -> &Uri {
        &self.0
    }
}

impl Header for Location {
    const NAME: &'static HeaderName = &header::LOCATION;

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().ok()?.parse().ok())
            .map(Location)
            .ok_or_else(Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(iter::once(self.into()))
    }
}

impl From<Uri> for Location {
    fn from(uri: Uri) -> Self {
        Location(uri)
    }
}

impl From<&Location> for HeaderValue {
    fn from(location: &Location) -> Self {
        location.0.to_string().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::*;

    #[test]
    fn absolute_uri() {
        let s = "http://www.example.net/index.html";
        let loc = test_decode::<Location>(&[s]).unwrap();

        assert_eq!(loc, Location(Uri::from_static(s)));
    }

    #[test]
    fn relative_uri_with_fragment() {
        let s = "/People.html#tim";
        let loc = test_decode::<Location>(&[s]).unwrap();

        assert_eq!(loc, Location(Uri::from_static(s)));
    }
}
