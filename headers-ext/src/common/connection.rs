use ::HeaderName;

/// `Connection` header, defined in
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-6.1)
///
/// The `Connection` header field allows the sender to indicate desired
/// control options for the current connection.  In order to avoid
/// confusing downstream recipients, a proxy or gateway MUST remove or
/// replace any received connection options before forwarding the
/// message.
///
/// # ABNF
///
/// ```text
/// Connection        = 1#connection-option
/// connection-option = token
///
/// # Example values
/// * `close`
/// * `keep-alive`
/// * `upgrade`
/// ```
///
/// # Examples
///
/// ```
/// use headers::Connection;
///
/// let keep_alive = Connection::keep_alive();
/// ```
///
/// ```
/// # extern crate headers;
/// # extern crate http;
/// use headers::Connection;
/// use http::header::HeaderName;
/// # fn main() {
///
/// let connection = Connection::new([HeaderName::from_static("upgrade")]);
/// # }
/// ```
#[derive(Clone, Debug, Header)]
#[header(csv)]
pub struct Connection(Vec<HeaderName>);

impl Connection {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=HeaderName>,
    {
        let headers = iter
            .into_iter()
            .collect();
        Connection(headers)
    }

    /// A constructor to easily create a `Connection: close` header.
    #[inline]
    pub fn close() -> Connection {
        Connection(vec![HeaderName::from_static("close")])
    }

    /// A constructor to easily create a `Connection: keep-alive` header.
    #[inline]
    pub fn keep_alive() -> Connection {
        Connection(vec![HeaderName::from_static("keep-alive")])
    }
}

/*
bench_header!(close, Connection, { vec![b"close".to_vec()] });
bench_header!(keep_alive, Connection, { vec![b"keep-alive".to_vec()] });
bench_header!(header, Connection, { vec![b"authorization".to_vec()] });
*/
