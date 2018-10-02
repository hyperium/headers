use std::time::SystemTime;
use util::HttpDate;

/// `If-Unmodified-Since` header, defined in
/// [RFC7232](http://tools.ietf.org/html/rfc7232#section-3.4)
///
/// The `If-Unmodified-Since` header field makes the request method
/// conditional on the selected representation's last modification date
/// being earlier than or equal to the date provided in the field-value.
/// This field accomplishes the same purpose as If-Match for cases where
/// the user agent does not have an entity-tag for the representation.
///
/// # ABNF
///
/// ```text
/// If-Unmodified-Since = HTTP-date
/// ```
///
/// # Example values
///
/// * `Sat, 29 Oct 1994 19:43:31 GMT`
///
/// # Example
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::IfUnmodifiedSince;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
/// let if_unmod = IfUnmodifiedSince::from(time);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Header)]
pub struct IfUnmodifiedSince(HttpDate);

impl From<SystemTime> for IfUnmodifiedSince {
    fn from(time: SystemTime) -> IfUnmodifiedSince {
        IfUnmodifiedSince(time.into())
    }
}

impl From<IfUnmodifiedSince> for SystemTime {
    fn from(date: IfUnmodifiedSince) -> SystemTime {
        date.0.into()
    }
}

