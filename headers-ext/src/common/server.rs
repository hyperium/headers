use http::header::HeaderValue;

/// `Server` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.4.2)
///
/// The `Server` header field contains information about the software
/// used by the origin server to handle the request, which is often used
/// by clients to help identify the scope of reported interoperability
/// problems, to work around or tailor requests to avoid particular
/// server limitations, and for analytics regarding server or operating
/// system use.  An origin server MAY generate a Server field in its
/// responses.
///
/// # ABNF
///
/// ```text
/// Server = product *( RWS ( product / comment ) )
/// ```
///
/// # Example values
/// * `CERN/3.0 libwww/2.17`
///
/// # Example
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::Server;
///
/// let server = Server::from_static("hyper/0.12.2");
/// ```
#[derive(Debug, Header)]
pub struct Server(HeaderValue);

impl Server {
    /// Construct a `Server` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub fn from_static(s: &'static str) -> Server {
        Server(HeaderValue::from_static(s))
    }
}

impl From<HeaderValue> for Server {
    fn from(value: HeaderValue) -> Server {
        Server(value)
    }
}
