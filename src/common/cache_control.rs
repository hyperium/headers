use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use http::{HeaderName, HeaderValue};

use crate::util::{self, Seconds};
use crate::{Error, Header};

/// `Cache-Control` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.2)
/// with extensions in [RFC8246](https://www.rfc-editor.org/rfc/rfc8246)
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
/// use headers::CacheControl;
///
/// let cc = CacheControl::new();
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct CacheControl {
    flags: Flags,
    max_age: Option<Seconds>,
    max_stale: Option<Seconds>,
    min_fresh: Option<Seconds>,
    s_max_age: Option<Seconds>,
}

#[derive(Debug, Clone, PartialEq)]
struct Flags {
    bits: u64,
}

impl Flags {
    const NO_CACHE: Self = Self { bits: 0b000000001 };
    const NO_STORE: Self = Self { bits: 0b000000010 };
    const NO_TRANSFORM: Self = Self { bits: 0b000000100 };
    const ONLY_IF_CACHED: Self = Self { bits: 0b000001000 };
    const MUST_REVALIDATE: Self = Self { bits: 0b000010000 };
    const PUBLIC: Self = Self { bits: 0b000100000 };
    const PRIVATE: Self = Self { bits: 0b001000000 };
    const PROXY_REVALIDATE: Self = Self { bits: 0b010000000 };
    const IMMUTABLE: Self = Self { bits: 0b100000000 };
    const MUST_UNDERSTAND: Self = Self { bits: 0b1000000000 };

    fn empty() -> Self {
        Self { bits: 0 }
    }

    fn contains(&self, flag: Self) -> bool {
        (self.bits & flag.bits) != 0
    }

    fn insert(&mut self, flag: Self) {
        self.bits |= flag.bits;
    }
}

impl CacheControl {
    /// Construct a new empty `CacheControl` header.
    pub fn new() -> Self {
        CacheControl {
            flags: Flags::empty(),
            max_age: None,
            max_stale: None,
            min_fresh: None,
            s_max_age: None,
        }
    }

    // getters

    /// Check if the `no-cache` directive is set.
    pub fn no_cache(&self) -> bool {
        self.flags.contains(Flags::NO_CACHE)
    }

    /// Check if the `no-store` directive is set.
    pub fn no_store(&self) -> bool {
        self.flags.contains(Flags::NO_STORE)
    }

    /// Check if the `no-transform` directive is set.
    pub fn no_transform(&self) -> bool {
        self.flags.contains(Flags::NO_TRANSFORM)
    }

    /// Check if the `only-if-cached` directive is set.
    pub fn only_if_cached(&self) -> bool {
        self.flags.contains(Flags::ONLY_IF_CACHED)
    }

    /// Check if the `public` directive is set.
    pub fn public(&self) -> bool {
        self.flags.contains(Flags::PUBLIC)
    }

    /// Check if the `private` directive is set.
    pub fn private(&self) -> bool {
        self.flags.contains(Flags::PRIVATE)
    }

    /// Check if the `immutable` directive is set.
    pub fn immutable(&self) -> bool {
        self.flags.contains(Flags::IMMUTABLE)
    }

    /// Check if the `must-revalidate` directive is set.
    pub fn must_revalidate(&self) -> bool {
        self.flags.contains(Flags::MUST_REVALIDATE)
    }

    /// Check if the `must-understand` directive is set.
    pub fn must_understand(&self) -> bool {
        self.flags.contains(Flags::MUST_UNDERSTAND)
    }

    /// Get the value of the `max-age` directive if set.
    pub fn max_age(&self) -> Option<Duration> {
        self.max_age.map(Into::into)
    }

    /// Get the value of the `max-stale` directive if set.
    pub fn max_stale(&self) -> Option<Duration> {
        self.max_stale.map(Into::into)
    }

    /// Get the value of the `min-fresh` directive if set.
    pub fn min_fresh(&self) -> Option<Duration> {
        self.min_fresh.map(Into::into)
    }

    /// Get the value of the `s-maxage` directive if set.
    pub fn s_max_age(&self) -> Option<Duration> {
        self.s_max_age.map(Into::into)
    }

    // setters

    /// Set the `no-cache` directive.
    pub fn with_no_cache(mut self) -> Self {
        self.flags.insert(Flags::NO_CACHE);
        self
    }

    /// Set the `no-store` directive.
    pub fn with_no_store(mut self) -> Self {
        self.flags.insert(Flags::NO_STORE);
        self
    }

    /// Set the `no-transform` directive.
    pub fn with_no_transform(mut self) -> Self {
        self.flags.insert(Flags::NO_TRANSFORM);
        self
    }

    /// Set the `only-if-cached` directive.
    pub fn with_only_if_cached(mut self) -> Self {
        self.flags.insert(Flags::ONLY_IF_CACHED);
        self
    }

    /// Set the `private` directive.
    pub fn with_private(mut self) -> Self {
        self.flags.insert(Flags::PRIVATE);
        self
    }

    /// Set the `public` directive.
    pub fn with_public(mut self) -> Self {
        self.flags.insert(Flags::PUBLIC);
        self
    }

    /// Set the `immutable` directive.
    pub fn with_immutable(mut self) -> Self {
        self.flags.insert(Flags::IMMUTABLE);
        self
    }

    /// Set the `must-revalidate` directive.
    pub fn with_must_revalidate(mut self) -> Self {
        self.flags.insert(Flags::MUST_REVALIDATE);
        self
    }

    /// Set the `must-understand` directive.
    pub fn with_must_understand(mut self) -> Self {
        self.flags.insert(Flags::MUST_UNDERSTAND);
        self
    }

    /// Set the `max-age` directive.
    pub fn with_max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration.into());
        self
    }

    /// Set the `max-stale` directive.
    pub fn with_max_stale(mut self, duration: Duration) -> Self {
        self.max_stale = Some(duration.into());
        self
    }

    /// Set the `min-fresh` directive.
    pub fn with_min_fresh(mut self, duration: Duration) -> Self {
        self.min_fresh = Some(duration.into());
        self
    }

    /// Set the `s-maxage` directive.
    pub fn with_s_max_age(mut self, duration: Duration) -> Self {
        self.s_max_age = Some(duration.into());
        self
    }
}

impl Header for CacheControl {
    fn name() -> &'static HeaderName {
        &::http::header::CACHE_CONTROL
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, Error> {
        let mut cache_control = CacheControl::new();

        for value in values {
            let string = match value.to_str() {
                Ok(string) => string,
                Err(_) => continue,
            };
            if string.as_bytes().contains(&b'"') {
                let mut in_quotes = false;
                for field in string.split(move |c| {
                    if in_quotes {
                        if c == '"' {
                            in_quotes = false;
                        }
                        false
                    } else if c == ',' {
                        true
                    } else {
                        if c == '"' {
                            in_quotes = true;
                        }
                        false
                    }
                }) {
                    cache_control.apply_field(field)?;
                }
            } else {
                for field in string.split(',') {
                    cache_control.apply_field(field)?;
                }
            }
        }

        Ok(cache_control)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(::std::iter::once(util::fmt(Fmt(self))));
    }
}

impl CacheControl {
    #[inline]
    fn apply_field(&mut self, field: &str) -> Result<(), Error> {
        let field = field.trim();
        if field.is_empty() {
            return Ok(());
        }
        let directive = match field.parse().map_err(|_| Error::invalid())? {
            KnownDirective::Known(directive) => directive,
            KnownDirective::Unknown => return Ok(()),
        };

        match directive {
            Directive::NoCache => {
                self.flags.insert(Flags::NO_CACHE);
            }
            Directive::NoStore => {
                self.flags.insert(Flags::NO_STORE);
            }
            Directive::NoTransform => {
                self.flags.insert(Flags::NO_TRANSFORM);
            }
            Directive::OnlyIfCached => {
                self.flags.insert(Flags::ONLY_IF_CACHED);
            }
            Directive::MustRevalidate => {
                self.flags.insert(Flags::MUST_REVALIDATE);
            }
            Directive::MustUnderstand => {
                self.flags.insert(Flags::MUST_UNDERSTAND);
            }
            Directive::Public => {
                self.flags.insert(Flags::PUBLIC);
            }
            Directive::Private => {
                self.flags.insert(Flags::PRIVATE);
            }
            Directive::Immutable => {
                self.flags.insert(Flags::IMMUTABLE);
            }
            Directive::ProxyRevalidate => {
                self.flags.insert(Flags::PROXY_REVALIDATE);
            }
            Directive::MaxAge(secs) => {
                self.max_age = Some(Duration::from_secs(secs).into());
            }
            Directive::MaxStale(secs) => {
                self.max_stale = Some(Duration::from_secs(secs).into());
            }
            Directive::MinFresh(secs) => {
                self.min_fresh = Some(Duration::from_secs(secs).into());
            }
            Directive::SMaxAge(secs) => {
                self.s_max_age = Some(Duration::from_secs(secs).into());
            }
        }
        Ok(())
    }
}

struct Fmt<'a>(&'a CacheControl);

impl fmt::Display for Fmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[inline]
        fn emit(f: &mut fmt::Formatter, first: &mut bool, directive: Directive) -> fmt::Result {
            if !*first {
                f.write_str(", ")?;
            }
            *first = false;
            fmt::Display::fmt(&directive, f)
        }

        let mut first = true;
        if self.0.flags.contains(Flags::NO_CACHE) {
            emit(f, &mut first, Directive::NoCache)?;
        }
        if self.0.flags.contains(Flags::NO_STORE) {
            emit(f, &mut first, Directive::NoStore)?;
        }
        if self.0.flags.contains(Flags::NO_TRANSFORM) {
            emit(f, &mut first, Directive::NoTransform)?;
        }
        if self.0.flags.contains(Flags::ONLY_IF_CACHED) {
            emit(f, &mut first, Directive::OnlyIfCached)?;
        }
        if self.0.flags.contains(Flags::MUST_REVALIDATE) {
            emit(f, &mut first, Directive::MustRevalidate)?;
        }
        if self.0.flags.contains(Flags::PUBLIC) {
            emit(f, &mut first, Directive::Public)?;
        }
        if self.0.flags.contains(Flags::PRIVATE) {
            emit(f, &mut first, Directive::Private)?;
        }
        if self.0.flags.contains(Flags::IMMUTABLE) {
            emit(f, &mut first, Directive::Immutable)?;
        }
        if self.0.flags.contains(Flags::MUST_UNDERSTAND) {
            emit(f, &mut first, Directive::MustUnderstand)?;
        }
        if self.0.flags.contains(Flags::PROXY_REVALIDATE) {
            emit(f, &mut first, Directive::ProxyRevalidate)?;
        }
        if let Some(seconds) = self.0.max_age.as_ref() {
            emit(f, &mut first, Directive::MaxAge(seconds.as_u64()))?;
        }
        if let Some(seconds) = self.0.max_stale.as_ref() {
            emit(f, &mut first, Directive::MaxStale(seconds.as_u64()))?;
        }
        if let Some(seconds) = self.0.min_fresh.as_ref() {
            emit(f, &mut first, Directive::MinFresh(seconds.as_u64()))?;
        }
        if let Some(seconds) = self.0.s_max_age.as_ref() {
            emit(f, &mut first, Directive::SMaxAge(seconds.as_u64()))?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum KnownDirective {
    Known(Directive),
    Unknown,
}

#[derive(Clone, Copy)]
enum Directive {
    NoCache,
    NoStore,
    NoTransform,
    OnlyIfCached,

    // request directives
    MaxAge(u64),
    MaxStale(u64),
    MinFresh(u64),

    // response directives
    MustRevalidate,
    MustUnderstand,
    Public,
    Private,
    Immutable,
    ProxyRevalidate,
    SMaxAge(u64),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(
            match *self {
                Directive::NoCache => "no-cache",
                Directive::NoStore => "no-store",
                Directive::NoTransform => "no-transform",
                Directive::OnlyIfCached => "only-if-cached",

                Directive::MaxAge(secs) => return write!(f, "max-age={}", secs),
                Directive::MaxStale(secs) => return write!(f, "max-stale={}", secs),
                Directive::MinFresh(secs) => return write!(f, "min-fresh={}", secs),

                Directive::MustRevalidate => "must-revalidate",
                Directive::MustUnderstand => "must-understand",
                Directive::Public => "public",
                Directive::Private => "private",
                Directive::Immutable => "immutable",
                Directive::ProxyRevalidate => "proxy-revalidate",
                Directive::SMaxAge(secs) => return write!(f, "s-maxage={}", secs),
            },
            f,
        )
    }
}

impl FromStr for KnownDirective {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(KnownDirective::Known(match s {
            "no-cache" => Directive::NoCache,
            "no-store" => Directive::NoStore,
            "no-transform" => Directive::NoTransform,
            "only-if-cached" => Directive::OnlyIfCached,
            "must-revalidate" => Directive::MustRevalidate,
            "public" => Directive::Public,
            "private" => Directive::Private,
            "immutable" => Directive::Immutable,
            "must-understand" => Directive::MustUnderstand,
            "proxy-revalidate" => Directive::ProxyRevalidate,
            "" => return Err(()),
            _ => match s.split_once('=') {
                Some((name, value)) if !value.is_empty() => match (name, value.trim_matches('"')) {
                    ("max-age", secs) => secs.parse().map(Directive::MaxAge).map_err(|_| ())?,
                    ("max-stale", secs) => secs.parse().map(Directive::MaxStale).map_err(|_| ())?,
                    ("min-fresh", secs) => secs.parse().map(Directive::MinFresh).map_err(|_| ())?,
                    ("s-maxage", secs) => secs.parse().map(Directive::SMaxAge).map_err(|_| ())?,
                    _unknown => return Ok(KnownDirective::Unknown),
                },
                Some(_) | None => return Ok(KnownDirective::Unknown),
            },
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn test_parse_multiple_headers() {
        assert_eq!(
            test_decode::<CacheControl>(&["no-cache", "private"]).unwrap(),
            CacheControl::new().with_no_cache().with_private(),
        );
    }

    #[test]
    fn decode_skips_non_string_header_values() {
        let invalid = HeaderValue::from_bytes(b"\x80").unwrap();
        let valid = HeaderValue::from_static("max-age=100");
        let cache_control = CacheControl::decode(&mut [&invalid, &valid].iter().copied()).unwrap();
        assert_eq!(cache_control.max_age(), Some(Duration::from_secs(100)));
    }

    #[test]
    fn test_parse_argument() {
        assert_eq!(
            test_decode::<CacheControl>(&["max-age=100, private"]).unwrap(),
            CacheControl::new()
                .with_max_age(Duration::from_secs(100))
                .with_private(),
        );
    }

    #[test]
    fn test_parse_quote_form() {
        assert_eq!(
            test_decode::<CacheControl>(&["max-age=\"200\""]).unwrap(),
            CacheControl::new().with_max_age(Duration::from_secs(200)),
        );
    }

    #[test]
    fn test_parse_quoted_comma() {
        assert_eq!(
            test_decode::<CacheControl>(&["foo=\"a, private, immutable, b\", no-cache"]).unwrap(),
            CacheControl::new().with_no_cache(),
            "unknown extensions are ignored but shouldn't fail parsing",
        )
    }

    #[test]
    fn test_parse_extension() {
        assert_eq!(
            test_decode::<CacheControl>(&["foo, no-cache, bar=baz"]).unwrap(),
            CacheControl::new().with_no_cache(),
            "unknown extensions are ignored but shouldn't fail parsing",
        );
    }

    #[test]
    fn test_immutable() {
        let cc = CacheControl::new().with_immutable();
        let headers = test_encode(cc.clone());
        assert_eq!(headers["cache-control"], "immutable");
        assert_eq!(test_decode::<CacheControl>(&["immutable"]).unwrap(), cc);
        assert!(cc.immutable());
    }

    #[test]
    fn test_must_revalidate() {
        let cc = CacheControl::new().with_must_revalidate();
        let headers = test_encode(cc.clone());
        assert_eq!(headers["cache-control"], "must-revalidate");
        assert_eq!(
            test_decode::<CacheControl>(&["must-revalidate"]).unwrap(),
            cc
        );
        assert!(cc.must_revalidate());
    }

    #[test]
    fn test_must_understand() {
        let cc = CacheControl::new().with_must_understand();
        let headers = test_encode(cc.clone());
        assert_eq!(headers["cache-control"], "must-understand");
        assert_eq!(
            test_decode::<CacheControl>(&["must-understand"]).unwrap(),
            cc
        );
        assert!(cc.must_understand());
    }

    #[test]
    fn test_parse_bad_syntax() {
        assert_eq!(test_decode::<CacheControl>(&["max-age=lolz"]), None);
    }

    #[test]
    fn encode_one_flag_directive() {
        let cc = CacheControl::new().with_no_cache();

        let headers = test_encode(cc);
        assert_eq!(headers["cache-control"], "no-cache");
    }

    #[test]
    fn encode_one_param_directive() {
        let cc = CacheControl::new().with_max_age(Duration::from_secs(300));

        let headers = test_encode(cc);
        assert_eq!(headers["cache-control"], "max-age=300");
    }

    #[test]
    fn encode_two_directive() {
        let headers = test_encode(CacheControl::new().with_no_cache().with_private());
        assert_eq!(headers["cache-control"], "no-cache, private");

        let headers = test_encode(
            CacheControl::new()
                .with_no_cache()
                .with_max_age(Duration::from_secs(100)),
        );
        assert_eq!(headers["cache-control"], "no-cache, max-age=100");
    }

    bench_header!(bench, CacheControl, "max-age=100, private, no-cache");
}
