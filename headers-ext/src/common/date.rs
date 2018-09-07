use util::HttpDate;

/// `Date` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.1.2)
///
/// The `Date` header field represents the date and time at which the
/// message was originated.
///
/// # ABNF
///
/// ```text
/// Date = HTTP-date
/// ```
///
/// # Example values
///
/// * `Tue, 15 Nov 1994 08:12:31 GMT`
///
/// # Example
///
/// ```
/// use headers::{Headers, Date};
/// use std::time::SystemTime;
///
/// let date = Date::from(SystemTime::now());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Header)]
pub struct Date(HttpDate);
