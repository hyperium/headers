use std::iter::FromIterator;

use mime::{self, Mime};

use {util::QualityValue, Header};

fn qitem(mime: Mime) -> QualityValue<Mime> {
    QualityValue::new(mime, Default::default())
}

/// `Accept` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.2)
///
/// The `Accept` header field can be used by user agents to specify
/// response media types that are acceptable.  Accept header fields can
/// be used to indicate that the request is specifically limited to a
/// small set of desired types, as in the case of a request for an
/// in-line image
///
/// # ABNF
///
/// ```text
/// Accept = #( media-range [ accept-params ] )
///
/// media-range    = ( "*/*"
///                  / ( type "/" "*" )
///                  / ( type "/" subtype )
///                  ) *( OWS ";" OWS parameter )
/// accept-params  = weight *( accept-ext )
/// accept-ext = OWS ";" OWS token [ "=" ( token / quoted-string ) ]
/// ```
///
/// # Example values
/// * `audio/*; q=0.2, audio/basic`
/// * `text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c`
///
/// # Examples
/// ```
/// # extern crate headers;
/// extern crate mime;
/// extern crate http;
/// use std::iter::FromIterator;
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
///
/// headers.typed_insert(
///     Accept::from_iter(vec![
///         QualityValue::new(mime::TEXT_HTML, Default::default()),
///     ])
/// );
/// ```
///
/// ```
/// # extern crate headers;
/// extern crate mime;
/// use std::iter::FromIterator;
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
/// headers.typed_insert(
///     Accept::from_iter(vec![
///         QualityValue::new(mime::APPLICATION_JSON, Default::default()),
///     ])
/// );
/// ```
/// ```
/// # extern crate headers;
/// extern crate mime;
/// use std::iter::FromIterator;
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
///
/// headers.typed_insert(
///     Accept::from_iter(vec![
///         QualityValue::from(mime::TEXT_HTML),
///         QualityValue::from("application/xhtml+xml".parse::<mime::Mime>().unwrap()),
///         QualityValue::new(
///             mime::TEXT_XML,
///             900.into()
///         ),
///         QualityValue::from("image/webp".parse::<mime::Mime>().unwrap()),
///         QualityValue::new(
///             mime::STAR_STAR,
///             800.into()
///         ),
///     ])
/// );
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Accept(Vec<QualityValue<Mime>>);

impl Header for Accept {
    fn name() -> &'static ::HeaderName {
        &::http::header::ACCEPT
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        ::util::csv::from_comma_delimited(values).map(Accept)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        use std::fmt;
        struct Format<F>(F);
        impl<F> fmt::Display for Format<F>
        where
            F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                (self.0)(f)
            }
        }
        let s = format!(
            "{}",
            Format(
                |f: &mut fmt::Formatter<'_>| ::util::csv::fmt_comma_delimited(
                    &mut *f,
                    self.0.iter()
                )
            )
        );
        values.extend(Some(::HeaderValue::from_str(&s).unwrap()))
    }
}

impl FromIterator<QualityValue<Mime>> for Accept {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = QualityValue<Mime>>,
    {
        Accept(iter.into_iter().collect())
    }
}

impl Accept {
    /// A constructor to easily create `Accept: */*`.
    pub fn star() -> Accept {
        Accept(vec![qitem(mime::STAR_STAR)])
    }

    /// A constructor to easily create `Accept: application/json`.
    pub fn json() -> Accept {
        Accept(vec![qitem(mime::APPLICATION_JSON)])
    }

    /// A constructor to easily create `Accept: text/*`.
    pub fn text() -> Accept {
        Accept(vec![qitem(mime::TEXT_STAR)])
    }

    /// A constructor to easily create `Accept: image/*`.
    pub fn image() -> Accept {
        Accept(vec![qitem(mime::IMAGE_STAR)])
    }

    /// Returns an iterator over the quality values
    pub fn iter(&self) -> impl Iterator<Item = &QualityValue<Mime>> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use {
        http::HeaderValue,
        mime::{TEXT_HTML, TEXT_PLAIN, TEXT_PLAIN_UTF_8},
    };

    use util::Quality;

    macro_rules! test_header {
        ($name: ident, $input: expr, $expected: expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Accept::decode(
                        &mut $input
                            .into_iter()
                            .map(|s| HeaderValue::from_bytes(s).unwrap())
                            .collect::<Vec<_>>()
                            .iter()
                    )
                    .ok(),
                    $expected,
                );
            }
        };
    }

    // Tests from the RFC
    test_header!(
        test1,
        vec![b"audio/*; q=0.2, audio/basic"],
        Some(Accept(vec![
            QualityValue::new("audio/*".parse().unwrap(), Quality::from(200)),
            qitem("audio/basic".parse().unwrap()),
        ]))
    );
    test_header!(
        test2,
        vec![b"text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c"],
        Some(Accept(vec![
            QualityValue::new(TEXT_PLAIN, Quality::from(500)),
            qitem(TEXT_HTML),
            QualityValue::new("text/x-dvi".parse().unwrap(), Quality::from(800)),
            qitem("text/x-c".parse().unwrap()),
        ]))
    );
    // Custom tests
    test_header!(
        test3,
        vec![b"text/plain; charset=utf-8"],
        Some(Accept(vec![qitem(TEXT_PLAIN_UTF_8),]))
    );
    test_header!(
        test4,
        vec![b"text/plain; charset=utf-8; q=0.5"],
        Some(Accept(vec![QualityValue::new(
            TEXT_PLAIN_UTF_8,
            Quality::from(500)
        ),]))
    );

    #[test]
    #[ignore]
    fn test_fuzzing1() {
        let raw = HeaderValue::from_static("chunk#;e");
        let header = Accept::decode(&mut Some(&raw).into_iter());
        assert!(header.is_ok());
    }
}
