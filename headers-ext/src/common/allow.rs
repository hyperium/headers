use http::Method;

/// `Allow` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.4.1)
///
/// The `Allow` header field lists the set of methods advertised as
/// supported by the target resource.  The purpose of this field is
/// strictly to inform the recipient of valid request methods associated
/// with the resource.
///
/// # ABNF
///
/// ```text
/// Allow = #method
/// ```
///
/// # Example values
/// * `GET, HEAD, PUT`
/// * `OPTIONS, GET, PUT, POST, DELETE, HEAD, TRACE, CONNECT, PATCH, fOObAr`
/// * ``
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// extern crate http;
/// use headers::Allow;
/// use http::Method;
///
/// let allow = Allow::new([Method::GET]);
/// ```
#[derive(Clone, Debug, PartialEq, Header)]
pub struct Allow(Vec<Method>);

impl Allow {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=Method>,
    {
        let methods = iter
            .into_iter()
            .collect();

        Allow(methods)
    }
}
