use std::fmt;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use http::header::HeaderValue;
use time::{PrimitiveDateTime, UtcOffset, Date, OffsetDateTime};

use super::IterExt;

/// A timestamp with HTTP formatting and parsing
//   Prior to 1995, there were three different formats commonly used by
//   servers to communicate timestamps.  For compatibility with old
//   implementations, all three are defined here.  The preferred format is
//   a fixed-length and single-zone subset of the date and time
//   specification used by the Internet Message Format [RFC5322].
//
//     HTTP-date    = IMF-fixdate / obs-date
//
//   An example of the preferred format is
//
//     Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
//
//   Examples of the two obsolete formats are
//
//     Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
//     Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
//
//   A recipient that parses a timestamp value in an HTTP header field
//   MUST accept all three HTTP-date formats.  When a sender generates a
//   header field that contains one or more timestamps defined as
//   HTTP-date, the sender MUST generate those timestamps in the
//   IMF-fixdate format.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HttpDate(OffsetDateTime);

impl HttpDate {
    pub(crate) fn from_val(val: &HeaderValue) -> Option<Self> {
        val.to_str().ok()?.parse().ok()
    }

    pub(crate) fn parse_gmt_date(s: &str, format: &str) -> Result<OffsetDateTime, time::ParseError> {
        PrimitiveDateTime::parse(s, format)
            .map(|t| t.assume_utc().to_offset(UtcOffset::UTC))
            // Handle malformed "abbreviated" dates like Chromium. See cookie#162.
            .map(|date| {
                let offset = match date.year() {
                    0..=68 => 2000,
                    69..=99 => 1900,
                    _ => return date,
                };
    
                let new_date = Date::try_from_ymd(date.year() + offset, date.month(), date.day());
                PrimitiveDateTime::new(new_date.expect("date from date"), date.time()).assume_utc()
            })
    }
}

// TODO: remove this and FromStr?
#[derive(Debug)]
pub struct Error(());

impl super::TryFromValues for HttpDate {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(HttpDate::from_val)
            .ok_or_else(::Error::invalid)
    }
}

impl From<HttpDate> for HeaderValue {
    fn from(date: HttpDate) -> HeaderValue {
        (&date).into()
    }
}

impl<'a> From<&'a HttpDate> for HeaderValue {
    fn from(date: &'a HttpDate) -> HeaderValue {
        // TODO: could be just BytesMut instead of String
        let s = date.to_string();
        let bytes = Bytes::from(s);
        HeaderValue::from_maybe_shared(bytes).expect("HttpDate always is a valid value")
    }
}

impl FromStr for HttpDate {
    type Err = Error;
    fn from_str(s: &str) -> Result<HttpDate, Error> {
        Self::parse_gmt_date(s, "%a, %d %b %Y %T GMT")
            .or_else(|_| Self::parse_gmt_date(s, "%A, %d-%b-%y %T GMT"))
            .or_else(|_| Self::parse_gmt_date(s, "%c"))
            .map(HttpDate)
            .map_err(|_| Error(()))
    }
}

impl fmt::Debug for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.format("D, d M y H:i:s O"), f)
    }
}

impl fmt::Display for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.format("D, d M y H:i:s O"), f)
    }
}

impl From<SystemTime> for HttpDate {
    fn from(sys: SystemTime) -> HttpDate {
        let odt = match sys.duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                // subsec nanos always dropped
                OffsetDateTime::from_unix_timestamp(dur.as_secs() as _)
            }
            Err(err) => {
                let neg = err.duration();
                // subsec nanos always dropped
                OffsetDateTime::from_unix_timestamp(-(neg.as_secs() as i64))
            }
        };
        HttpDate(odt)
    }
}

impl From<HttpDate> for SystemTime {
    fn from(date: HttpDate) -> SystemTime {
        let odt = date.0;
        if odt.timestamp() >= 0 {
            UNIX_EPOCH + Duration::new(odt.second().into(), odt.nanosecond().into())
        } else {
            UNIX_EPOCH - Duration::new(odt.second().into(), odt.nanosecond().into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HttpDate;

    #[inline]
    fn nov_07() -> HttpDate {
        HttpDate(HttpDate::parse_gmt_date("Sun Nov 07 08:48:37 1994", "%c").unwrap())
    }

    #[test]
    fn test_imf_fixdate() {
        assert_eq!(
            "Sun, 07 Nov 1994 08:48:37 GMT".parse::<HttpDate>().unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_rfc_850() {
        assert_eq!(
            "Sunday, 07-Nov-94 08:48:37 GMT"
                .parse::<HttpDate>()
                .unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_asctime() {
        assert_eq!(
            "Sun Nov 7 08:48:37 1994".parse::<HttpDate>().unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_no_date() {
        assert!("this-is-no-date".parse::<HttpDate>().is_err());
    }
}
