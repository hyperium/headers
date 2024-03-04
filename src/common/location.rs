use std::convert::TryFrom;

use http::Uri;
use HeaderValue;

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
#[derive(Clone, Debug, PartialEq)]
pub struct Location(HeaderValue);

derive_header! {
    Location(_),
    name: LOCATION
}

impl Location {
    /// Accesses the header's value
    pub fn value(&self) -> &HeaderValue {
        &self.0
    }
}

impl From<Uri> for Location {
    fn from(uri: Uri) -> Self {
        Self(
            HeaderValue::try_from(uri.to_string())
                // cf. https://www.rfc-editor.org/rfc/rfc3986#section-2
                .expect("All URI characters should be valid HTTP header value characters"),
        )
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

        assert_eq!(loc, Location(HeaderValue::from_static(s)));
    }

    #[test]
    fn relative_uri_with_fragment() {
        let s = "/People.html#tim";
        let loc = test_decode::<Location>(&[s]).unwrap();

        assert_eq!(loc, Location(HeaderValue::from_static(s)));
    }

    #[test]
    fn uri_constructor() {
        let s = "https://www.rust-lang.org/tools";
        let uri: Uri = s.parse().unwrap();
        let loc = Location::from(uri);

        assert_eq!(loc, Location(HeaderValue::from_static(s)));
        assert_eq!(loc.value().to_str().unwrap(), s);
    }

    #[test]
    fn uri_constructor_invalid_chars() {
        let s = "https://www.rust-lang.org/hélas";
        let uri: Result<Uri, _> = s.parse();
        assert!(uri.is_err());
    }
}
