use http::HeaderValue;

/// `Upgrade` header, defined in [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-6.7)
///
/// The `Upgrade` header field is intended to provide a simple mechanism
/// for transitioning from HTTP/1.1 to some other protocol on the same
/// connection.  A client MAY send a list of protocols in the Upgrade
/// header field of a request to invite the server to switch to one or
/// more of those protocols, in order of descending preference, before
/// sending the final response.  A server MAY ignore a received Upgrade
/// header field if it wishes to continue using the current protocol on
/// that connection.  Upgrade cannot be used to insist on a protocol
/// change.
///
/// ## ABNF
///
/// ```text
/// Upgrade          = 1#protocol
///
/// protocol         = protocol-name ["/" protocol-version]
/// protocol-name    = token
/// protocol-version = token
/// ```
///
/// ## Example values
///
/// * `HTTP/2.0, SHTTP/1.3, IRC/6.9, RTA/x11`
///
/// # Note
///
/// In practice, the `Upgrade` header is never that complicated. In most cases,
/// it is only ever a single value, such as `"websocket"`.
///
/// # Examples
///
/// ```
/// use headers::Upgrade;
///
/// let ws = Upgrade::websocket();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Upgrade(HeaderValue);

derive_header! {
    Upgrade(_),
    name: UPGRADE
}

impl Upgrade {
    /// Constructs an `Upgrade: websocket` header.
    pub fn websocket() -> Upgrade {
        Upgrade(HeaderValue::from_static("websocket"))
    }

    /// Constructs an `Upgrade: h2c` header, for HTTP/2 over TCP. There's no
    /// constructor for HTTP/2 over TLS, as using ALPN or prior knowledge is
    /// better-suited to that usecase.
    pub fn h2c() -> Upgrade {
        Upgrade(HeaderValue::from_static("h2c"))
    }

    /// Constructs an `Upgrade` header with the given `HeaderValue`
    pub fn new(value: HeaderValue) -> Upgrade {
        Upgrade(value)
    }

    /// Returns the header value, e.g. "websocket" or "h2c", or a
    /// comma-separated list, see RFC 7230:
    /// <https://datatracker.ietf.org/doc/html/rfc7230#section-6.7>
    ///
    /// If the header value cannot be represented as a utf-8 string,
    /// `None` is returned.
    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::*;

    #[test]
    fn websocket() {
        let s = "websocket";
        let loc = test_decode::<Upgrade>(&[s]).unwrap();

        assert_eq!(loc.to_str(), Some(s));
    }
}
