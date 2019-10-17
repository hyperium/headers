use http::HttpTryFrom;

use mime::{self, Mime};

use util::QualityValue;

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
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
///
/// headers.typed_insert(
///     Accept(vec![
///         QualityValue::new(mime::TEXT_HTML, Default::default()),
///     ])
/// );
/// ```
///
/// ```
/// # extern crate headers;
/// extern crate mime;
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
/// headers.typed_insert(
///     Accept(vec![
///         QualityValue::new(mime::APPLICATION_JSON, Default::default()),
///     ])
/// );
/// ```
/// ```
/// # extern crate headers;
/// extern crate mime;
/// use headers::{Accept, QualityValue, HeaderMapExt};
///
/// let mut headers = http::HeaderMap::new();
///
/// headers.typed_insert(
///     Accept(vec![
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
#[derive(Debug)]
pub struct Accept(pub Vec<QualityValue<Mime>>);

impl ::Header for Accept {
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
        values.extend(Some(::HeaderValue::try_from(s).unwrap()))
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
}
