use ::{HeaderName};

/// `Vary` header, defined in [RFC7231](https://tools.ietf.org/html/rfc7231#section-7.1.4)
///
/// The "Vary" header field in a response describes what parts of a
/// request message, aside from the method, Host header field, and
/// request target, might influence the origin server's process for
/// selecting and representing this response.  The value consists of
/// either a single asterisk ("*") or a list of header field names
/// (case-insensitive).
///
/// # ABNF
///
/// ```text
/// Vary = "*" / 1#field-name
/// ```
///
/// # Example values
///
/// * `accept-encoding, accept-language`
///
/// # Example
///
/// ```
/// use headers::{Headers, Vary};
///
/// let mut headers = Headers::new();
/// headers.set(Vary::Any);
/// ```
///
/// # Example
///
/// ```
/// # extern crate headers;
/// # extern crate unicase;
/// # fn main() {
/// // extern crate unicase;
///
/// use headers::{Headers, Vary};
/// use unicase::Ascii;
///
/// let mut headers = Headers::new();
/// headers.set(
///     Vary::Items(vec![
///         Ascii::new("accept-encoding".to_owned()),
///         Ascii::new("accept-language".to_owned()),
///     ])
/// );
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Header)]
#[header(csv)]
pub struct Vary(Vary_);

#[derive(Debug, Clone, PartialEq)]
enum Vary_ {
    Any,
    Items(Vec<HeaderName>),
}

enum Iter<'a> {
    Any(Option<()>),
    Items(::std::slice::Iter<'a, HeaderName>),
}

impl Vary {
    pub const ANY: Vary = Vary(Vary_::Any);
}

impl Vary_ {
    fn into_iter(&self) -> Iter {
        match self {
            Vary_::Any => Iter::Any(Some(())),
            Vary_::Items(ref items) => Iter::Items(items.into_iter()),
        }

    }
}


impl<'a> Iterator for Iter<'a> {
    type Item = &'a &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Any(ref mut star) => star.take().map(|_| &"*"),
            Iter::Items(ref mut items) => items.next().map(|val| &val.as_str()),
        }
    }
}

/*
test_vary {
    test_header!(test1, vec![b"accept-encoding, accept-language"]);

    #[test]
    fn test2() {
        let mut vary: ::Result<Vary>;

        vary = Header::parse_header(&"*".into());
        assert_eq!(vary.ok(), Some(Vary::Any));

        vary = Header::parse_header(&"etag,cookie,allow".into());
        assert_eq!(vary.ok(), Some(Vary::Items(vec!["eTag".parse().unwrap(),
                                                    "cookIE".parse().unwrap(),
                                                    "AlLOw".parse().unwrap(),])));
    }
}
*/
