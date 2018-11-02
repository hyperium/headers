use std::fmt;

use bytes::Bytes;
use http::uri::Authority;

/// The `Host` header.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct Host(Authority);

impl Host {
    /// Get the hostname, such as example.domain.
    pub fn hostname(&self) -> &str {
        self.0.host()
    }

    /// Get the optional port number.
    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }
}

impl ::Header for Host {
    const NAME: &'static ::HeaderName = &::http::header::HOST;

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Option<Self> {
        let value = Bytes::from(values.next()?.clone());

        Authority::from_shared(value)
            .ok()
            .map(Host)
    }

    fn encode(&self, values: &mut ::ToValues) {
        let bytes = Bytes::from(self.0.clone());

        let val = ::HeaderValue::from_shared(bytes)
            .expect("Authority is a valid HeaderValue");

        values.append(val);
    }
}

impl From<Authority> for Host {
    fn from(auth: Authority) -> Host {
        Host(auth)
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

