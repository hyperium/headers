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

impl IfUnmodifiedSince {
    /// Check if the supplied time passes the precondtion.
    pub fn precondition_passes(&self, last_modified: SystemTime) -> bool {
        let if_unmod = SystemTime::from(self.0);
        last_modified <= if_unmod
    }
}

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

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn precondition_passes() {
        let now = SystemTime::now();
        let one_sec_ago = now - Duration::from_secs(1);
        let two_sec_ago = now - Duration::from_secs(2);

        let if_unmod = IfUnmodifiedSince::from(one_sec_ago);
        assert!(!if_unmod.precondition_passes(now));
        assert!(if_unmod.precondition_passes(one_sec_ago));
        assert!(if_unmod.precondition_passes(two_sec_ago));
    }
}
