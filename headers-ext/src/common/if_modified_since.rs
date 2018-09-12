use std::time::SystemTime;
use util::HttpDate;

/// `If-Modified-Since` header, defined in
/// [RFC7232](http://tools.ietf.org/html/rfc7232#section-3.3)
///
/// The `If-Modified-Since` header field makes a GET or HEAD request
/// method conditional on the selected representation's modification date
/// being more recent than the date provided in the field-value.
/// Transfer of the selected representation's data is avoided if that
/// data has not changed.
///
/// # ABNF
///
/// ```text
/// If-Unmodified-Since = HTTP-date
/// ```
///
/// # Example values
/// * `Sat, 29 Oct 1994 19:43:31 GMT`
///
/// # Example
///
/// ```
/// use headers::{Headers, IfModifiedSince};
/// use std::time::{SystemTime, Duration};
///
/// let mut headers = Headers::new();
/// let modified = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
/// headers.set(IfModifiedSince(modified.into()));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Header)]
pub struct IfModifiedSince(HttpDate);

impl From<SystemTime> for IfModifiedSince {
    fn from(time: SystemTime) -> IfModifiedSince {
        IfModifiedSince(time.into())
    }
}

impl From<IfModifiedSince> for SystemTime {
    fn from(date: IfModifiedSince) -> SystemTime {
        date.0.into()
    }
}