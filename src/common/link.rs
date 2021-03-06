//! Link header and types.

use std::fmt;
use std::borrow::Cow;
use std::str::FromStr;
#[allow(unused, deprecated)]
use std::ascii::AsciiExt;

use mime::Mime;
use language_tags::LanguageTag;

/// The `Link` header, defined in
/// [RFC5988](http://tools.ietf.org/html/rfc5988#section-5)
///
/// # ABNF
///
/// ```text
/// Link           = "Link" ":" #link-value
/// link-value     = "<" URI-Reference ">" *( ";" link-param )
/// link-param     = ( ( "rel" "=" relation-types )
///                | ( "anchor" "=" <"> URI-Reference <"> )
///                | ( "rev" "=" relation-types )
///                | ( "hreflang" "=" Language-Tag )
///                | ( "media" "=" ( MediaDesc | ( <"> MediaDesc <"> ) ) )
///                | ( "title" "=" quoted-string )
///                | ( "title*" "=" ext-value )
///                | ( "type" "=" ( media-type | quoted-mt ) )
///                | ( link-extension ) )
/// link-extension = ( parmname [ "=" ( ptoken | quoted-string ) ] )
///                | ( ext-name-star "=" ext-value )
/// ext-name-star  = parmname "*" ; reserved for RFC2231-profiled
/// ; extensions.  Whitespace NOT
/// ; allowed in between.
/// ptoken         = 1*ptokenchar
/// ptokenchar     = "!" | "#" | "$" | "%" | "&" | "'" | "("
///                | ")" | "*" | "+" | "-" | "." | "/" | DIGIT
///                | ":" | "<" | "=" | ">" | "?" | "@" | ALPHA
///                | "[" | "]" | "^" | "_" | "`" | "{" | "|"
///                | "}" | "~"
/// media-type     = type-name "/" subtype-name
/// quoted-mt      = <"> media-type <">
/// relation-types = relation-type
///                | <"> relation-type *( 1*SP relation-type ) <">
/// relation-type  = reg-rel-type | ext-rel-type
/// reg-rel-type   = LOALPHA *( LOALPHA | DIGIT | "." | "-" )
/// ext-rel-type   = URI
/// ```
///
/// # Example values
///
/// `Link: <http://example.com/TheBook/chapter2>; rel="previous";
///        title="previous chapter"`
///
/// `Link: </TheBook/chapter2>; rel="previous"; title*=UTF-8'de'letztes%20Kapitel,
///        </TheBook/chapter4>; rel="next"; title*=UTF-8'de'n%c3%a4chstes%20Kapitel`
///
/// # Examples
///
/// ```
/// use headers::{HeaderMap, HeaderMapExt, link::{Link, LinkValue, RelationType}};
///
/// let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
///     .push_rel(RelationType::PREVIOUS)
///     .set_title("previous chapter");
///
/// let mut headers = HeaderMap::new();
/// headers.typed_insert(
///     Link::new(vec![link_value])
/// );
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Link {
    /// A list of the `link-value`s of the Link entity-header.
    values: Vec<LinkValue>
}

/// A single `link-value` of a `Link` header, based on:
/// [RFC5988](http://tools.ietf.org/html/rfc5988#section-5)
#[derive(Clone, PartialEq, Debug)]
pub struct LinkValue {
    /// Target IRI: `link-value`.
    link: Cow<'static, str>,

    /// Forward Relation Types: `rel`.
    rel: Option<Vec<RelationType>>,

    /// Context IRI: `anchor`.
    anchor: Option<String>,

    /// Reverse Relation Types: `rev`.
    rev: Option<Vec<RelationType>>,

    /// Hint on the language of the result of dereferencing
    /// the link: `hreflang`.
    href_lang: Option<Vec<LanguageTag>>,

    /// Destination medium or media: `media`.
    media_desc: Option<Vec<MediaDesc>>,

    /// Label of the destination of a Link: `title`.
    title: Option<String>,

    /// The `title` encoded in a different charset: `title*`.
    title_star: Option<String>,

    /// Hint on the media type of the result of dereferencing
    /// the link: `type`.
    media_type: Option<Mime>,
}

////////////////////////////////////////////////////////////////////////////////
// Typed variants
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_variants {
    (
        $(#[$attrs:meta])*
        name: $name:ident,
        mod_name: $mod_name:ident,
        $($typed:ident => $string:expr,)*
    ) => {
        mod $mod_name {
            use std::{fmt, str::FromStr};

            $(#[$attrs])*
            #[derive(Clone, PartialEq, Debug)]
            pub struct $name(Inner);

            impl $name {
                $(
                    #[doc = $string]
                    pub const $typed: $name = $name(Inner::$typed);
                )*
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match &self.0 {
                        $(Inner::$typed => fmt::Display::fmt($string, f),)*
                        Inner::Custom(custom) => fmt::Display::fmt(&custom, f),
                    }
                }
            }

            impl FromStr for $name {
                type Err = ::Error;

                fn from_str(input: &str) -> Result<$name, ::Error> {
                    if false {
                        unreachable!();
                    }
                    $(else if $string.eq_ignore_ascii_case(input) {
                        Ok(Self(Inner::$typed))
                    })*
                    else {
                        Ok(Self(Inner::Custom(input.into())))
                    }
                }
            }

            #[derive(Clone, PartialEq, Debug)]
            #[allow(nonstandard_style)]
            enum Inner {
                $($typed,)*
                Custom(String),
            }
        }

        pub use self::$mod_name::$name;
    }
}


impl_variants! {
    /// A Media Descriptor, based on:
    /// [https://www.w3.org/TR/html401/types.html#h-6.13][url]
    ///
    /// [url]: https://www.w3.org/TR/html401/types.html#h-6.13
    name: MediaDesc,
    mod_name: media_desc,

    SCREEN => "screen",
    TTY => "tty",
    TV => "tv",
    PROJECTION => "projection",
    HANDHELD => "handheld",
    PRINT => "print",
    BRAILLE => "braille",
    AURAL => "aural",
    ALL => "all",
}

impl_variants! {
    /// A Link Relation Type, based on:
    /// [RFC5988](https://tools.ietf.org/html/rfc5988#section-6.2.2)
    name: RelationType,
    mod_name: relation_type,

    ALTERNATE => "alternate",
    APPENDIX => "appendix",
    BOOKMARK => "bookmark",
    CHAPTER => "chapter",
    CONTENTS => "contents",
    COPYRIGHT => "copyright",
    CURRENT => "current",
    DESCRIBED_BY => "described-by",
    EDIT => "edit",
    EDIT_MEDIA => "edit-media",
    ENCLOSURE => "enclosure",
    FIRST => "first",
    GLOSSARY => "glossary",
    HELP => "help",
    HUB => "hub",
    INDEX => "index",
    LAST => "last",
    LATEST_VERSION => "latest-version",
    LICENSE => "license",
    NEXT => "next",
    NEXT_ARCHIVE => "next-archive",
    PAYMENT => "payment",
    PREV => "prev",
    PREDECESSOR_VERSION => "predecessor-version",
    PREVIOUS => "previous",
    PREV_ARCHIVE => "prev-archive",
    RELATED => "related",
    REPLIES => "replies",
    SECTION => "section",
    SELF => "self",
    SERVICE => "service",
    START => "start",
    STYLESHEET => "stylesheet",
    SUBSECTION => "subsection",
    SUCCESSOR_VERSION => "successor-version",
    UP => "up",
    VERSION_HISTORY => "version-history",
    VIA => "via",
    WORKING_COPY => "working-copy",
    WORKING_COPY_OF => "working-copy-of",
}

////////////////////////////////////////////////////////////////////////////////
// Struct methods
////////////////////////////////////////////////////////////////////////////////

impl Link {
    /// Create `Link` from a `Vec<LinkValue>`.
    pub fn new(link_values: Vec<LinkValue>) -> Link {
        Link { values: link_values }
    }

    /// Get the `Link` header's `LinkValue`s.
    pub fn values(&self) -> &[LinkValue] {
        self.values.as_ref()
    }

    /// Add a `LinkValue` instance to the `Link` header's values.
    pub fn push_value(&mut self, link_value: LinkValue) {
        self.values.push(link_value);
    }
}

impl LinkValue {
    /// Create `LinkValue` from URI-Reference.
    pub fn new<T>(uri: T) -> LinkValue
        where T: Into<Cow<'static, str>> {
        LinkValue {
            link: uri.into(),
            rel: None,
            anchor: None,
            rev: None,
            href_lang: None,
            media_desc: None,
            title: None,
            title_star: None,
            media_type: None,
        }
    }

    /// Get the `LinkValue`'s value.
    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    /// Get the `LinkValue`'s `rel` parameter(s).
    pub fn rel(&self) -> Option<&[RelationType]> {
        self.rel.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `anchor` parameter.
    pub fn anchor(&self) -> Option<&str> {
        self.anchor.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `rev` parameter(s).
    pub fn rev(&self) -> Option<&[RelationType]> {
        self.rev.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `hreflang` parameter(s).
    pub fn href_lang(&self) -> Option<&[LanguageTag]> {
        self.href_lang.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `media` parameter(s).
    pub fn media_desc(&self) -> Option<&[MediaDesc]> {
        self.media_desc.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `title` parameter.
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `title*` parameter.
    pub fn title_star(&self) -> Option<&str> {
        self.title_star.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `type` parameter.
    pub fn media_type(&self) -> Option<&Mime> {
        self.media_type.as_ref()
    }

    /// Add a `RelationType` to the `LinkValue`'s `rel` parameter.
    pub fn push_rel(mut self, rel: RelationType) -> LinkValue {
        let mut v = self.rel.take().unwrap_or(Vec::new());

        v.push(rel);

        self.rel = Some(v);

        self
    }

    /// Set `LinkValue`'s `anchor` parameter.
    pub fn set_anchor<T: Into<String>>(mut self, anchor: T) -> LinkValue {
        self.anchor = Some(anchor.into());

        self
    }

    /// Add a `RelationType` to the `LinkValue`'s `rev` parameter.
    pub fn push_rev(mut self, rev: RelationType) -> LinkValue {
        let mut v = self.rev.take().unwrap_or(Vec::new());

        v.push(rev);

        self.rev = Some(v);

        self
    }

    /// Add a `LanguageTag` to the `LinkValue`'s `hreflang` parameter.
    pub fn push_href_lang(mut self, language_tag: LanguageTag) -> LinkValue {
        let mut v = self.href_lang.take().unwrap_or(Vec::new());

        v.push(language_tag);

        self.href_lang = Some(v);

        self
    }

    /// Add a `MediaDesc` to the `LinkValue`'s `media_desc` parameter.
    pub fn push_media_desc(mut self, media_desc: MediaDesc) -> LinkValue {
        let mut v = self.media_desc.take().unwrap_or(Vec::new());

        v.push(media_desc);

        self.media_desc = Some(v);

        self
    }

    /// Set `LinkValue`'s `title` parameter.
    pub fn set_title<T: Into<String>>(mut self, title: T) -> LinkValue {
        self.title = Some(title.into());

        self
    }

    /// Set `LinkValue`'s `title*` parameter.
    pub fn set_title_star<T: Into<String>>(mut self, title_star: T) -> LinkValue {
        self.title_star = Some(title_star.into());

        self
    }

    /// Set `LinkValue`'s `type` parameter.
    pub fn set_media_type(mut self, media_type: Mime) -> LinkValue {
        self.media_type = Some(media_type);

        self
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait implementations
////////////////////////////////////////////////////////////////////////////////

impl ::Header for Link {
    fn name() -> &'static ::HeaderName {
        &::http::header::LINK
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i ::HeaderValue>,
    {
        // If more that one `Link` headers are present in a request's
        // headers they are combined in a single `Link` header containing
        // all the `link-value`s present in each of those `Link` headers.
        values
            .map(|line| {
                line.to_str()
                    .map_err(|_| ::Error::invalid())
                    .and_then(|line| Link::from_str(line))
            })
            .fold(None, |p, c| {
                match (p, c) {
                    (None, c) => Some(c),
                    (e @ Some(Err(_)), _) => e,
                    (Some(Ok(mut p)), Ok(c)) => {
                        p.values.extend(c.values);

                        Some(Ok(p))
                    },
                    _ => Some(Err(::Error::invalid())),
                }
            })
            .unwrap_or(Err(::Error::invalid()))
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(::HeaderValue::from_str(&self.to_string()).unwrap()));
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for value in &self.values {
            if !first {
                write!(f, ", ")?;
            }
            first = false;

            write!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl fmt::Display for LinkValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.link)?;

        if let Some(ref rel) = self.rel {
            fmt_delimited(f, rel.as_slice(), " ", ("; rel=\"", "\""))?;
        }
        if let Some(ref anchor) = self.anchor {
            write!(f, "; anchor=\"{}\"", anchor)?;
        }
        if let Some(ref rev) = self.rev {
            fmt_delimited(f, rev.as_slice(), " ", ("; rev=\"", "\""))?;
        }
        if let Some(ref href_lang) = self.href_lang {
            for tag in href_lang {
                write!(f, "; hreflang={}", tag)?;
            }
        }
        if let Some(ref media_desc) = self.media_desc {
            fmt_delimited(f, media_desc.as_slice(), ", ", ("; media=\"", "\""))?;
        }
        if let Some(ref title) = self.title {
            write!(f, "; title=\"{}\"", title)?;
        }
        if let Some(ref title_star) = self.title_star {
            write!(f, "; title*={}", title_star)?;
        }
        if let Some(ref media_type) = self.media_type {
            write!(f, "; type=\"{}\"", media_type)?;
        }

        Ok(())
    }
}

impl FromStr for Link {
    type Err = ::Error;

    fn from_str(s: &str) -> Result<Link, ::Error> {
        // Create a split iterator with delimiters: `;`, `,`
        let link_split = SplitAsciiUnquoted::new(s, ";,");

        let mut link_values: Vec<LinkValue> = Vec::new();

        // Loop over the splits parsing the Link header into
        // a `Vec<LinkValue>`
        for segment in link_split {
            // Parse the `Target IRI`
            // https://tools.ietf.org/html/rfc5988#section-5.1
            if segment.trim().starts_with('<') {
                link_values.push(
                    match verify_and_trim(segment.trim(), (b'<', b'>')) {
                        Err(_) => return Err(::Error::invalid()),
                        Ok(s) => {
                            LinkValue {
                                link: s.to_owned().into(),
                                rel: None,
                                anchor: None,
                                rev: None,
                                href_lang: None,
                                media_desc: None,
                                title: None,
                                title_star: None,
                                media_type: None,
                            }
                        },
                    }
                );
            } else {
                // Parse the current link-value's parameters
                let mut link_param_split = segment.splitn(2, '=');

                let link_param_name = match link_param_split.next() {
                    None => return Err(::Error::invalid()),
                    Some(p) => p.trim(),
                };

                let link_header = match link_values.last_mut() {
                    None => return Err(::Error::invalid()),
                    Some(l) => l,
                };

                if "rel".eq_ignore_ascii_case(link_param_name) {
                    // Parse relation type: `rel`.
                    // https://tools.ietf.org/html/rfc5988#section-5.3
                    if link_header.rel.is_none() {
                        link_header.rel = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(' ')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<RelationType>, _>>()
                                    .or_else(|_| Err(::Error::invalid()))
                                    .ok()
                            },
                        };
                    }
                } else if "anchor".eq_ignore_ascii_case(link_param_name) {
                    // Parse the `Context IRI`.
                    // https://tools.ietf.org/html/rfc5988#section-5.2
                    link_header.anchor = match link_param_split.next() {
                        None | Some("") => return Err(::Error::invalid()),
                        Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                            Err(_) => return Err(::Error::invalid()),
                            Ok(a) => Some(String::from(a)),
                        },
                    };
                } else if "rev".eq_ignore_ascii_case(link_param_name) {
                    // Parse relation type: `rev`.
                    // https://tools.ietf.org/html/rfc5988#section-5.3
                    if link_header.rev.is_none() {
                        link_header.rev = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(' ')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<RelationType>, _>>()
                                    .or_else(|_| Err(::Error::invalid()))
                                    .ok()
                            },
                        }
                    }
                } else if "hreflang".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `hreflang`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    let mut v = link_header.href_lang.take().unwrap_or(Vec::new());

                    v.push(
                        match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => match s.trim().parse() {
                                Err(_) => return Err(::Error::invalid()),
                                Ok(t) => t,
                            },
                        }
                    );

                    link_header.href_lang = Some(v);
                } else if "media".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `media`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.media_desc.is_none() {
                        link_header.media_desc = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(',')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<MediaDesc>, _>>()
                                    .or_else(|_| Err(::Error::invalid()))
                                    .ok()
                            },
                        };
                    }
                } else if "title".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `title`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.title.is_none() {
                        link_header.title = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                                Err(_) => return Err(::Error::invalid()),
                                Ok(t) => Some(String::from(t)),
                            },
                        };
                    }
                } else if "title*".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `title*`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    //
                    // Definition of `ext-value`:
                    //       https://tools.ietf.org/html/rfc5987#section-3.2.1
                    if link_header.title_star.is_none() {
                        link_header.title_star = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => Some(String::from(s.trim())),
                        };
                    }
                } else if "type".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `type`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.media_type.is_none() {
                        link_header.media_type = match link_param_split.next() {
                            None | Some("") => return Err(::Error::invalid()),
                            Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                                Err(_) => return Err(::Error::invalid()),
                                Ok(t) => match t.parse() {
                                    Err(_) => return Err(::Error::invalid()),
                                    Ok(m) => Some(m),
                                },
                            },

                        };
                    }
                } else {
                    return Err(::Error::invalid());
                }
            }
        }

        Ok(Link::new(link_values))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Utilities
////////////////////////////////////////////////////////////////////////////////

struct SplitAsciiUnquoted<'a> {
    src: &'a str,
    pos: usize,
    del: &'a str
}

impl<'a> SplitAsciiUnquoted<'a> {
    fn new(s: &'a str, d: &'a str) -> SplitAsciiUnquoted<'a> {
        SplitAsciiUnquoted{
            src: s,
            pos: 0,
            del: d,
        }
    }
}

impl<'a> Iterator for SplitAsciiUnquoted<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.pos < self.src.len() {
            let prev_pos = self.pos;
            let mut pos = self.pos;

            let mut in_quotes = false;

            for c in self.src[prev_pos..].as_bytes().iter() {
                in_quotes ^= *c == b'"';

                // Ignore `c` if we're `in_quotes`.
                if !in_quotes && self.del.as_bytes().contains(c) {
                    break;
                }

                pos += 1;
            }

            self.pos = pos + 1;

            Some(&self.src[prev_pos..pos])
        } else {
            None
        }
    }
}

fn fmt_delimited<T: fmt::Display>(f: &mut fmt::Formatter, p: &[T], d: &str, b: (&str, &str)) -> fmt::Result {
    if p.len() != 0 {
        // Write a starting string `b.0` before the first element
        write!(f, "{}{}", b.0, p[0])?;

        for i in &p[1..] {
            // Write the next element preceded by the delimiter `d`
            write!(f, "{}{}", d, i)?;
        }

        // Write a ending string `b.1` before the first element
        write!(f, "{}", b.1)?;
    }

    Ok(())
}

fn verify_and_trim(s: &str, b: (u8, u8)) -> Result<&str, ::Error> {
    let length = s.len();
    let byte_array = s.as_bytes();

    // Verify that `s` starts with `b.0` and ends with `b.1` and return
    // the contained substring after trimming whitespace.
    if length > 1 && b.0 == byte_array[0] && b.1 == byte_array[length - 1] {
        Ok(s.trim_matches(
            |c: char| c == b.0 as char || c == b.1 as char || c.is_whitespace())
        )
    } else {
        Err(::Error::invalid())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::Write;

    use super::{Link, LinkValue, MediaDesc, RelationType, SplitAsciiUnquoted};
    use super::{fmt_delimited, verify_and_trim};

    use Header;

    use mime;

    fn parse_header(values: &[&[u8]]) -> Result<Link, ::Error> {
        let values = values.iter()
            .map(|val| ::HeaderValue::from_bytes(val).expect("invalid header value"))
            .collect::<Vec<_>>();
        Link::decode(&mut values.iter())
    }

    #[test]
    fn test_link() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .push_rev(RelationType::NEXT)
            .set_title("previous chapter");

        let link_header = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; title=\"previous chapter\"";

        let expected_link = Link::new(vec![link_value]);

        let link = parse_header(&[link_header]);
        assert_eq!(link.ok(), Some(expected_link));
    }

    #[test]
    fn test_link_multiple_values() {
        let first_link = LinkValue::new("/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .set_title_star("UTF-8'de'letztes%20Kapitel");

        let second_link = LinkValue::new("/TheBook/chapter4")
            .push_rel(RelationType::NEXT)
            .set_title_star("UTF-8'de'n%c3%a4chstes%20Kapitel");

        let link_header = b"</TheBook/chapter2>; \
            rel=\"previous\"; title*=UTF-8'de'letztes%20Kapitel, \
            </TheBook/chapter4>; \
            rel=\"next\"; title*=UTF-8'de'n%c3%a4chstes%20Kapitel";

        let expected_link = Link::new(vec![first_link, second_link]);

        let link = parse_header(&[link_header]);
        assert_eq!(link.ok(), Some(expected_link));
    }

    #[test]
    fn test_link_all_attributes() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .set_anchor("../anchor/example/")
            .push_rev(RelationType::NEXT)
            .push_href_lang("de".parse().unwrap())
            .push_media_desc(MediaDesc::SCREEN)
            .set_title("previous chapter")
            .set_title_star("title* unparsed")
            .set_media_type(mime::TEXT_PLAIN);

        let link_header = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; anchor=\"../anchor/example/\"; \
            rev=\"next\"; hreflang=de; media=\"screen\"; \
            title=\"previous chapter\"; title*=title* unparsed; \
            type=\"text/plain\"";

        let expected_link = Link::new(vec![link_value]);

        let link = parse_header(&[link_header]);
        assert_eq!(link.ok(), Some(expected_link));
    }

    #[test]
    fn test_link_multiple_link_headers() {
        let first_link = LinkValue::new("/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .set_title_star("UTF-8'de'letztes%20Kapitel");

        let second_link = LinkValue::new("/TheBook/chapter4")
            .push_rel(RelationType::NEXT)
            .set_title_star("UTF-8'de'n%c3%a4chstes%20Kapitel");

        let third_link = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .push_rev(RelationType::NEXT)
            .set_title("previous chapter");

        let expected_link = Link::new(vec![first_link, second_link, third_link]);

        let link = parse_header(&[
            b"</TheBook/chapter2>; rel=\"previous\"; title*=UTF-8'de'letztes%20Kapitel, \
              </TheBook/chapter4>; rel=\"next\"; title*=UTF-8'de'n%c3%a4chstes%20Kapitel",
            b"<http://example.com/TheBook/chapter2>; rel=\"previous\"; rev=next; \
              title=\"previous chapter\"",
        ]).unwrap();

        assert_eq!(link, expected_link);
    }

    #[test]
    fn test_link_display() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::PREVIOUS)
            .set_anchor("/anchor/example/")
            .push_rev(RelationType::NEXT)
            .push_href_lang("de".parse().unwrap())
            .push_media_desc(MediaDesc::SCREEN)
            .set_title("previous chapter")
            .set_title_star("title* unparsed")
            .set_media_type(mime::TEXT_PLAIN);

        let link = Link::new(vec![link_value]);

        let mut link_header = String::new();
        write!(&mut link_header, "{}", link).unwrap();

        let expected_link_header = "<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; anchor=\"/anchor/example/\"; \
            rev=\"next\"; hreflang=de; media=\"screen\"; \
            title=\"previous chapter\"; title*=title* unparsed; \
            type=\"text/plain\"";

        assert_eq!(link_header, expected_link_header);
    }

    #[test]
    fn test_link_parsing_errors() {
        let link_a  = b"http://example.com/TheBook/chapter2; \
            rel=\"previous\"; rev=next; title=\"previous chapter\"";

        let mut err: Result<Link, _> = parse_header(&[link_a]);
        assert_eq!(err.is_err(), true);

        let link_b = b"<http://example.com/TheBook/chapter2>; \
            =\"previous\"; rev=next; title=\"previous chapter\"";

        err = parse_header(&[link_b]);
        assert_eq!(err.is_err(), true);

        let link_c = b"<http://example.com/TheBook/chapter2>; \
            rel=; rev=next; title=\"previous chapter\"";

        err = parse_header(&[link_c]);
        assert_eq!(err.is_err(), true);

        let link_d = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; title=";

        err = parse_header(&[link_d]);
        assert_eq!(err.is_err(), true);

        let link_e = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; attr=unknown";

        err = parse_header(&[link_e]);
        assert_eq!(err.is_err(), true);
     }

    #[test]
    fn test_link_split_ascii_unquoted_iterator() {
        let string = "some, text; \"and, more; in quotes\", or not";
        let mut string_split = SplitAsciiUnquoted::new(string, ";,");

        assert_eq!(Some("some"), string_split.next());
        assert_eq!(Some(" text"), string_split.next());
        assert_eq!(Some(" \"and, more; in quotes\""), string_split.next());
        assert_eq!(Some(" or not"), string_split.next());
        assert_eq!(None, string_split.next());
    }

    #[test]
    fn test_link_fmt_delimited() {
        struct TestFormatterStruct<'a> { v: Vec<&'a str> };

        impl<'a> fmt::Display for TestFormatterStruct<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt_delimited(f, self.v.as_slice(), ", ", (">>", "<<"))
            }
        }

        let test_formatter = TestFormatterStruct { v: vec!["first", "second"] };

        let mut string = String::new();
        write!(&mut string, "{}", test_formatter).unwrap();

        let expected_string = ">>first, second<<";

        assert_eq!(string, expected_string);
    }

    #[test]
    fn test_link_verify_and_trim() {
        let string = verify_and_trim(">  some string   <", (b'>', b'<'));
        assert_eq!(string.ok(), Some("some string"));

        let err = verify_and_trim(" >  some string   <", (b'>', b'<'));
        assert_eq!(err.is_err(), true);
    }
}
