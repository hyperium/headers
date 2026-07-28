use std::fmt;

use http::HeaderValue;

use super::{FlatCsv, IterExt};
use crate::Error;

/// An entity tag, defined in [RFC7232](https://tools.ietf.org/html/rfc7232#section-2.3)
///
/// An entity tag consists of a string enclosed by two literal double quotes.
/// Preceding the first double quote is an optional weakness indicator,
/// which always looks like `W/`. Examples for valid tags are `"xyzzy"` and `W/"xyzzy"`.
///
/// # ABNF
///
/// ```text
/// entity-tag = [ weak ] opaque-tag
/// weak       = %x57.2F ; "W/", case-sensitive
/// opaque-tag = DQUOTE *etagc DQUOTE
/// etagc      = %x21 / %x23-7E / obs-text
///            ; VCHAR except double quotes, plus obs-text
/// ```
///
/// # Comparison
/// To check if two entity tags are equivalent in an application always use the `strong_eq` or
/// `weak_eq` methods based on the context of the Tag. Only use `==` to check if two tags are
/// identical.
///
/// The example below shows the results for a set of entity-tag pairs and
/// both the weak and strong comparison function results:
///
/// | ETag 1  | ETag 2  | Strong Comparison | Weak Comparison |
/// |---------|---------|-------------------|-----------------|
/// | `W/"1"` | `W/"1"` | no match          | match           |
/// | `W/"1"` | `W/"2"` | no match          | no match        |
/// | `W/"1"` | `"1"`   | no match          | match           |
/// | `"1"`   | `"1"`   | match             | match           |
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct EntityTag<T = HeaderValue>(T);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EntityTagRange {
    Any,
    Tags(FlatCsv),
}

// ===== impl EntityTag =====

impl<T: AsRef<[u8]>> EntityTag<T> {
    /// Get the tag.
    pub(crate) fn tag(&self) -> &[u8] {
        let bytes = self.0.as_ref();
        debug_assert!(bytes.len() >= 2 && bytes.last() == Some(&b'"'));
        let start = if self.is_weak() { 3 } else { 1 };
        debug_assert!(start < bytes.len());

        &bytes[start..bytes.len() - 1]
    }

    /// Return if this is a "weak" tag.
    pub(crate) fn is_weak(&self) -> bool {
        let bytes = self.0.as_ref();
        debug_assert!(!bytes.is_empty());

        bytes[0] == b'W'
    }

    /// For strong comparison two entity-tags are equivalent if both are not weak and their
    /// opaque-tags match character-by-character.
    pub(crate) fn strong_eq<R>(&self, other: &EntityTag<R>) -> bool
    where
        R: AsRef<[u8]>,
    {
        !self.is_weak() && !other.is_weak() && self.tag() == other.tag()
    }

    /// For weak comparison two entity-tags are equivalent if their
    /// opaque-tags match character-by-character, regardless of either or
    /// both being tagged as "weak".
    pub(crate) fn weak_eq<R>(&self, other: &EntityTag<R>) -> bool
    where
        R: AsRef<[u8]>,
    {
        self.tag() == other.tag()
    }

    /// The inverse of `EntityTag.strong_eq()`.
    #[cfg(test)]
    pub(crate) fn strong_ne(&self, other: &EntityTag) -> bool {
        !self.strong_eq(other)
    }

    /// The inverse of `EntityTag.weak_eq()`.
    #[cfg(test)]
    pub(crate) fn weak_ne(&self, other: &EntityTag) -> bool {
        !self.weak_eq(other)
    }

    #[inline]
    pub(crate) fn parse(src: T) -> Option<Self> {
        let slice = src.as_ref();
        let length = slice.len();

        // Early exits if it doesn't terminate in a DQUOTE.
        if length < 2 || slice[length - 1] != b'"' {
            return None;
        }

        let start = match slice[0] {
            // "<tag>"
            b'"' => 1,
            // W/"<tag>"
            b'W' => {
                if length >= 4 && slice[1] == b'/' && slice[2] == b'"' {
                    3
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        if check_slice_validity(&slice[start..length - 1]) {
            Some(EntityTag(src))
        } else {
            None
        }
    }
}

impl EntityTag {
    /*
    /// Constructs a new EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn new(weak: bool, tag: String) -> EntityTag {
        assert!(check_slice_validity(&tag), "Invalid tag: {:?}", tag);
        EntityTag { weak: weak, tag: tag }
    }

    /// Constructs a new weak EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn weak(tag: String) -> EntityTag {
        EntityTag::new(true, tag)
    }

    /// Constructs a new strong EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn strong(tag: String) -> EntityTag {
        EntityTag::new(false, tag)
    }
    */

    #[cfg(test)]
    pub fn from_static(bytes: &'static str) -> EntityTag {
        let val = HeaderValue::from_static(bytes);
        match EntityTag::from_val(&val) {
            Some(tag) => tag,
            None => {
                panic!("invalid static string for EntityTag: {:?}", bytes);
            }
        }
    }

    #[inline]
    pub(crate) fn from_owned(val: HeaderValue) -> Option<EntityTag> {
        EntityTag::parse(val.as_bytes())?;
        Some(EntityTag(val))
    }

    #[inline]
    pub(crate) fn from_val(val: &HeaderValue) -> Option<EntityTag> {
        EntityTag::parse(val.as_bytes()).map(|_entity| EntityTag(val.clone()))
    }
}

impl<T: fmt::Debug> fmt::Debug for EntityTag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl super::TryFromValues for EntityTag {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(EntityTag::from_val)
            .ok_or_else(Error::invalid)
    }
}

impl From<EntityTag> for HeaderValue {
    fn from(tag: EntityTag) -> HeaderValue {
        tag.0
    }
}

impl<'a> From<&'a EntityTag> for HeaderValue {
    fn from(tag: &'a EntityTag) -> HeaderValue {
        tag.0.clone()
    }
}

/// check that each char in the slice is either:
/// 1. `%x21`, or
/// 2. in the range `%x23` to `%x7E`, or
/// 3. above `%x80`
fn check_slice_validity(slice: &[u8]) -> bool {
    debug_assert!(slice
        .iter()
        .all(|&c| (b'\x21'..=b'\x7e').contains(&c) | (c >= b'\x80')));
    !slice.contains(&b'"')
}

// ===== impl EntityTagRange =====

impl EntityTagRange {
    pub(crate) fn matches_strong(&self, entity: &EntityTag) -> bool {
        self.matches_if(entity, |a, b| a.strong_eq(b))
    }

    pub(crate) fn matches_weak(&self, entity: &EntityTag) -> bool {
        self.matches_if(entity, |a, b| a.weak_eq(b))
    }

    fn matches_if<F>(&self, entity: &EntityTag, func: F) -> bool
    where
        F: Fn(&EntityTag<&str>, &EntityTag) -> bool,
    {
        match *self {
            EntityTagRange::Any => true,
            EntityTagRange::Tags(ref tags) => {
                if let Ok(value) = tags.value.to_str() {
                    if let Some(matches) = matches_valid_list(value, entity, &func) {
                        return matches;
                    }
                }

                tags.iter()
                    .flat_map(EntityTag::<&str>::parse)
                    .any(|tag| func(&tag, entity))
            }
        }
    }
}

fn matches_valid_list<F>(value: &str, entity: &EntityTag, func: &F) -> Option<bool>
where
    F: Fn(&EntityTag<&str>, &EntityTag) -> bool,
{
    let bytes = value.as_bytes();
    let mut position = 0;

    loop {
        while matches!(bytes.get(position), Some(b' ' | b'\t')) {
            position += 1;
        }
        if position == bytes.len() {
            return Some(false);
        }

        let start = position;
        if bytes.get(position..position + 3) == Some(b"W/\"") {
            position += 3;
        } else if bytes.get(position) == Some(&b'"') {
            position += 1;
        } else {
            return None;
        }

        let close = bytes[position..].iter().position(|&byte| byte == b'"')?;
        position += close + 1;
        let end = position;

        while matches!(bytes.get(position), Some(b' ' | b'\t')) {
            position += 1;
        }
        let has_more = match bytes.get(position) {
            None => false,
            Some(b',') => {
                position += 1;
                true
            }
            Some(_) => return None,
        };

        let raw_tag = &value[start..end];
        debug_assert!(EntityTag::parse(raw_tag).is_some());
        let tag = EntityTag(raw_tag);
        if func(&tag, entity) {
            return Some(true);
        }
        if !has_more {
            return Some(false);
        }
    }
}

impl super::TryFromValues for EntityTagRange {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let flat = FlatCsv::try_from_values(values)?;
        if flat.value == "*" {
            Ok(EntityTagRange::Any)
        } else {
            Ok(EntityTagRange::Tags(flat))
        }
    }
}

impl<'a> From<&'a EntityTagRange> for HeaderValue {
    fn from(tag: &'a EntityTagRange) -> HeaderValue {
        match *tag {
            EntityTagRange::Any => HeaderValue::from_static("*"),
            EntityTagRange::Tags(ref tags) => tags.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(slice: &[u8]) -> Option<EntityTag> {
        let val = HeaderValue::from_bytes(slice).ok()?;
        EntityTag::from_val(&val)
    }

    #[test]
    fn test_etag_parse_success() {
        // Expected success
        let tag = parse(b"\"foobar\"").unwrap();
        assert!(!tag.is_weak());
        assert_eq!(tag.tag(), b"foobar");

        let weak = parse(b"W/\"weaktag\"").unwrap();
        assert!(weak.is_weak());
        assert_eq!(weak.tag(), b"weaktag");
    }

    #[test]
    fn test_etag_parse_failures() {
        // Expected failures
        macro_rules! fails {
            ($slice:expr) => {
                assert_eq!(parse($slice), None);
            };
        }

        fails!(b"no-dquote");
        fails!(b"w/\"the-first-w-is-case sensitive\"");
        fails!(b"W/\"");
        fails!(b"");
        fails!(b"\"unmatched-dquotes1");
        fails!(b"unmatched-dquotes2\"");
        fails!(b"\"inner\"quotes\"");
    }

    /*
    #[test]
    fn test_etag_fmt() {
        assert_eq!(format!("{}", EntityTag::strong("foobar".to_owned())), "\"foobar\"");
        assert_eq!(format!("{}", EntityTag::strong("".to_owned())), "\"\"");
        assert_eq!(format!("{}", EntityTag::weak("weak-etag".to_owned())), "W/\"weak-etag\"");
        assert_eq!(format!("{}", EntityTag::weak("\u{0065}".to_owned())), "W/\"\x65\"");
        assert_eq!(format!("{}", EntityTag::weak("".to_owned())), "W/\"\"");
    }
    */

    #[test]
    fn test_cmp() {
        // | ETag 1  | ETag 2  | Strong Comparison | Weak Comparison |
        // |---------|---------|-------------------|-----------------|
        // | `W/"1"` | `W/"1"` | no match          | match           |
        // | `W/"1"` | `W/"2"` | no match          | no match        |
        // | `W/"1"` | `"1"`   | no match          | match           |
        // | `"1"`   | `"1"`   | match             | match           |
        let mut etag1 = EntityTag::from_static("W/\"1\"");
        let mut etag2 = etag1.clone();
        assert!(!etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));

        etag2 = EntityTag::from_static("W/\"2\"");
        assert!(!etag1.strong_eq(&etag2));
        assert!(!etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(etag1.weak_ne(&etag2));

        etag2 = EntityTag::from_static("\"1\"");
        assert!(!etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));

        etag1 = EntityTag::from_static("\"1\"");
        assert!(etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(!etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));
    }

    #[test]
    fn tag_list_does_not_match_valid_prefix_with_invalid_suffix() {
        let entity = EntityTag::from_static("\"target\"");

        for value in [
            "\"target\"junk",
            "W/\"target\"junk",
            "\"other\", \"target\"junk",
        ] {
            let range = EntityTagRange::Tags(HeaderValue::from_static(value).into());
            assert!(!range.matches_strong(&entity), "{:?}", value);
            assert!(!range.matches_weak(&entity), "{:?}", value);
        }
    }
}
