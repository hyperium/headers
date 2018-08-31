use http::uri::Authority;

/// The `Host` header.
///
/// HTTP/1.1 requires that all requests include a `Host` header, and so hyper
/// client requests add one automatically.
///
/// # Examples
/// ```
/// use headers::{Headers, Host};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Host::new("hyper.rs", None)
/// );
/// ```
/// ```
/// use headers::{Headers, Host};
///
/// let mut headers = Headers::new();
/// headers.set(
///     // In Rust 1.12+
///     // Host::new("hyper.rs", 8080)
///     Host::new("hyper.rs", Some(8080))
/// );
/// ```
#[derive(Clone, PartialEq, Debug, Header)]
pub struct Host(Authority);

impl Host {
    /*
    /// Create a `Host` header, providing the hostname and optional port.
    pub fn new<H, P>(hostname: H, port: P) -> Host
    where H: Into<Cow<'static, str>>,
          P: Into<Option<u16>>
    {
        Host {
            hostname: hostname.into(),
            port: port.into(),
        }
    }

    /// Get the hostname, such as example.domain.
    pub fn hostname(&self) -> &str {
        self.hostname.as_ref()
    }

    /// Get the optional port number.
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    */
}

/*
impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.port {
            None | Some(80) | Some(443) => f.write_str(&self.hostname[..]),
            Some(port) => write!(f, "{}:{}", self.hostname, port)
        }
    }
}
*/

