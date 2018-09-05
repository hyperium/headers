use std::fmt;
use std::str::FromStr;
use {Header, ToValues, Values};
use headers_core::{decode, encode};

/// `Cache-Control` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.2)
///
/// The `Cache-Control` header field is used to specify directives for
/// caches along the request/response chain.  Such cache directives are
/// unidirectional in that the presence of a directive in a request does
/// not imply that the same directive is to be given in the response.
///
/// ## ABNF
///
/// ```text
/// Cache-Control   = 1#cache-directive
/// cache-directive = token [ "=" ( token / quoted-string ) ]
/// ```
///
/// ## Example values
///
/// * `no-cache`
/// * `private, community="UCI"`
/// * `max-age=30`
///
/// # Example
///
/// ```
/// use headers::{CacheControl, CacheDirective};
///
/// let cc = CacheControl::new([CacheDirective::NO_CACHE]);
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct CacheControl {
    directives: Vec<CacheDirective>,
}

impl CacheControl {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=CacheDirective>,
    {
        let directives = iter
            .into_iter()
            .collect();

        CacheControl {
            directives,
        }
    }
}

impl Header for CacheControl {
    const NAME: &'static ::http::header::HeaderName = &::http::header::CACHE_CONTROL;

    fn decode(values: &mut Values) -> ::headers_core::Result<CacheControl> {
        decode::from_comma_delimited(values)
            .map(|directives: Vec<CacheDirective>| {
                debug_assert!(!directives.is_empty());
                CacheControl {
                    directives,
                }
            })
    }

    fn encode(&self, values: &mut ToValues) {
        values.append_fmt(self)
    }
}

impl fmt::Display for CacheControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        encode::comma_delimited(f, self.directives.iter())
    }
}

/// `CacheControl` contains a list of these directives.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CacheDirective(Directive);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Directive {
    /// "no-cache"
    NoCache,
    /// "no-store"
    NoStore,
    /// "no-transform"
    NoTransform,
    /// "only-if-cached"
    OnlyIfCached,

    // request directives
    /// "max-age=delta"
    MaxAge(u32),
    /// "max-stale=delta"
    MaxStale(u32),
    /// "min-fresh=delta"
    MinFresh(u32),

    // response directives
    /// "must-revalidate"
    MustRevalidate,
    /// "public"
    Public,
    /// "private"
    Private,
    /// "proxy-revalidate"
    ProxyRevalidate,
    /// "s-maxage=delta"
    SMaxAge(u32),

    /// Extension directives. Optionally include an argument.
    Extension(String, Option<String>)
}

impl CacheDirective {
    /// "no-cache"
    pub const NO_CACHE: Self = CacheDirective(Directive::NoCache);

    /// "no-store"
    pub const NO_STORE: Self = CacheDirective(Directive::NoStore);

    /// "no-transform"
    pub const NO_TRANSFORM: Self = CacheDirective(Directive::NoTransform);

    /// "only-if-cached"
    pub const ONLY_IF_CACHED: Self = CacheDirective(Directive::OnlyIfCached);

    /// "must-revalidate"
    pub const MUST_REVALIDATE: Self = CacheDirective(Directive::MustRevalidate);

    /// "public"
    pub const PUBLIC: Self = CacheDirective(Directive::Public);

    /// "private"
    pub const PRIVATE: Self = CacheDirective(Directive::Private);

    /// "proxy-revalidate"
    pub const PROXY_REVALIDATE: Self = CacheDirective(Directive::ProxyRevalidate);

    //TODO: accept Duration instead?
    /// "max-age=delta"
    pub fn max_age(age: u32) -> Self {
        CacheDirective(Directive::MaxAge(age))
    }

    /// "max-stale=delta"
    pub fn max_stale(age: u32) -> Self {
        CacheDirective(Directive::MaxStale(age))
    }

    /// "min-fresh=delta"
    pub fn min_fresh(age: u32) -> Self {
        CacheDirective(Directive::MinFresh(age))
    }

    /// "s-maxage=delta"
    pub fn s_max_age(age: u32) -> Self {
        CacheDirective(Directive::SMaxAge(age))
    }
}

impl fmt::Display for CacheDirective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(match self.0 {
            Directive::NoCache => "no-cache",
            Directive::NoStore => "no-store",
            Directive::NoTransform => "no-transform",
            Directive::OnlyIfCached => "only-if-cached",

            Directive::MaxAge(secs) => return write!(f, "max-age={}", secs),
            Directive::MaxStale(secs) => return write!(f, "max-stale={}", secs),
            Directive::MinFresh(secs) => return write!(f, "min-fresh={}", secs),

            Directive::MustRevalidate => "must-revalidate",
            Directive::Public => "public",
            Directive::Private => "private",
            Directive::ProxyRevalidate => "proxy-revalidate",
            Directive::SMaxAge(secs) => return write!(f, "s-maxage={}", secs),

            Directive::Extension(ref name, None) => &name[..],
            Directive::Extension(ref name, Some(ref arg)) => return write!(f, "{}={}", name, arg),

        }, f)
    }
}

/// Error if `CacheDirective` fails to parse.
#[derive(Debug)]
pub struct FromStrErr(());

impl FromStr for CacheDirective {
    type Err = FromStrErr;
    fn from_str(s: &str) -> Result<CacheDirective, Self::Err> {
        let directive = match s {
            "no-cache" => Directive::NoCache,
            "no-store" => Directive::NoStore,
            "no-transform" => Directive::NoTransform,
            "only-if-cached" => Directive::OnlyIfCached,
            "must-revalidate" => Directive::MustRevalidate,
            "public" => Directive::Public,
            "private" => Directive::Private,
            "proxy-revalidate" => Directive::ProxyRevalidate,
            "" => return Err(FromStrErr(())),
            _ => match s.find('=') {
                Some(idx) if idx+1 < s.len() => match (&s[..idx], (&s[idx+1..]).trim_matches('"')) {
                    ("max-age" , secs) => secs.parse().map(Directive::MaxAge).map_err(|_| FromStrErr(()))?,
                    ("max-stale", secs) => secs.parse().map(Directive::MaxStale).map_err(|_| FromStrErr(()))?,
                    ("min-fresh", secs) => secs.parse().map(Directive::MinFresh).map_err(|_| FromStrErr(()))?,
                    ("s-maxage", secs) => secs.parse().map(Directive::SMaxAge).map_err(|_| FromStrErr(()))?,
                    (left, right) => Directive::Extension(left.to_owned(), Some(right.to_owned()))
                },
                Some(_) => return Err(FromStrErr(())),
                None => Directive::Extension(s.to_owned(), None),
            }
        };
        Ok(CacheDirective(directive))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_decode;

    #[test]
    fn test_parse_multiple_headers() {
        assert_eq!(
            test_decode::<CacheControl>(&["no-cache", "private"]).unwrap(),
            CacheControl::new(vec![
                CacheDirective::NO_CACHE,
                CacheDirective::PRIVATE,
            ]),
        );
    }

    #[test]
    fn test_parse_argument() {
        assert_eq!(
            test_decode::<CacheControl>(&["max-age=100, private"]).unwrap(),
            CacheControl::new(vec![
                CacheDirective::max_age(100),
                CacheDirective::PRIVATE,
            ]),
        );
    }

    #[test]
    fn test_parse_quote_form() {
        assert_eq!(
            test_decode::<CacheControl>(&["max-age=\"200\""]).unwrap(),
            CacheControl::new(vec![
                CacheDirective::max_age(200),
            ]),
        );
    }

    /* TODO
    #[test]
    fn test_parse_extension() {
        let cache = Header::parse_header(&vec![b"foo, bar=baz".to_vec()].into());
        assert_eq!(cache.ok(), Some(CacheControl(vec![
            CacheDirective::Extension("foo".to_owned(), None),
            CacheDirective::Extension("bar".to_owned(), Some("baz".to_owned()))])))
    }
    */

    #[test]
    fn test_parse_bad_syntax() {
        assert_eq!(
            test_decode::<CacheControl>(&["foo="]),
            None,
        );
    }
}

