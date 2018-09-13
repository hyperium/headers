use std::iter::FromIterator;

use util::FlatCsv;
use ::{HeaderName, HeaderValue};

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
/// # extern crate headers_ext as headers;
/// use headers::Connection;
///
/// let keep_alive = Connection::keep_alive();
/// ```
///
/// ```
/// # extern crate headers_ext as headers;
/// # extern crate http;
/// use headers::Connection;
/// use http::header::UPGRADE;
/// # fn main() {
///
/// let connection = [UPGRADE]
///     .into_iter()
///     .collect::<Connection>();
/// # }
/// ```
// This is frequently just 1 or 2 values, so optimize for that case.
#[derive(Clone, Debug, Header)]
pub struct Connection(FlatCsv);

impl Connection {
    /// A constructor to easily create a `Connection: close` header.
    #[inline]
    pub fn close() -> Connection {
        Connection(HeaderValue::from_static("close").into())
    }

    /// A constructor to easily create a `Connection: keep-alive` header.
    #[inline]
    pub fn keep_alive() -> Connection {
        Connection(HeaderValue::from_static("keep-alive").into())
    }
}

impl FromIterator<HeaderName> for Connection {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let flat = iter
            .into_iter()
            .map(HeaderValue::from)
            .collect();
        Connection(flat)
    }
}

/*
bench_header!(close, Connection, { vec![b"close".to_vec()] });
bench_header!(keep_alive, Connection, { vec![b"keep-alive".to_vec()] });
bench_header!(header, Connection, { vec![b"authorization".to_vec()] });
*/
