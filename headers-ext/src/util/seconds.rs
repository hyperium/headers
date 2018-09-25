use std::fmt;
use std::time::Duration;

use {HeaderValue};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds(Duration);

impl Seconds {
    pub(crate) fn from_val(val: &HeaderValue) -> Option<Self> {
        let secs = val
            .to_str()
            .ok()?
            .parse()
            .ok()?;

        Some(Seconds(Duration::from_secs(secs)))
    }
}

impl ::headers_core::decode::TryFromValues for Seconds {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        Seconds::from_val(values.next()?)
    }
}

impl<'a> From<&'a Seconds> for HeaderValue {
    fn from(secs: &'a Seconds) -> HeaderValue {
        secs.0.as_secs().into()
    }
}

impl From<Duration> for Seconds {
    fn from(dur: Duration) -> Seconds {
        Seconds(dur)
    }
}

impl fmt::Debug for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.0.as_secs())
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.as_secs(), f)
    }
}
