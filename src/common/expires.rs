use std::{fmt, time::SystemTime};

use crate::util::HttpDate;

/// `Expires` header, defined in [RFC7234](https://datatracker.ietf.org/doc/html/rfc7234#section-5.3)
///
/// The `Expires` header field gives the date/time after which the
/// response is considered stale.
///
/// The presence of an Expires field does not imply that the original
/// resource will change or cease to exist at, before, or after that
/// time.
///
/// # ABNF
///
/// ```text
/// Expires = HTTP-date
/// ```
///
/// # Example values
/// * `Thu, 01 Dec 1994 16:00:00 GMT`
///
/// # Example
///
/// ```
/// use headers::Expires;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() + Duration::from_secs(60 * 60 * 24);
/// let expires = Expires::from(time);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expires(HttpDate);

derive_header! {
    Expires(_),
    name: EXPIRES
}

impl From<SystemTime> for Expires {
    fn from(time: SystemTime) -> Expires {
        Expires(time.into())
    }
}

impl From<Expires> for SystemTime {
    fn from(date: Expires) -> SystemTime {
        date.0.into()
    }
}

impl fmt::Display for Expires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::*;

    fn expires(s: &str) -> Expires {
        test_decode(&[s]).unwrap()
    }

    #[test]
    fn format() {
        let s = "Thu, 01 Dec 1994 16:00:00 GMT";
        assert_eq!(expires(s).to_string(), s);
    }
}
