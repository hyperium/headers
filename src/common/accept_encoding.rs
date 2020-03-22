use util::QualityValue;
// use HeaderValue;


/// `Accept-Encoding` header, defined in
/// [RFC7231](https://tools.ietf.org/html/rfc7231#section-5.3.4)
/// 
#[derive(Clone, Debug)]
pub struct AcceptEncoding(QualityValue);

derive_header! {
    AcceptEncoding(_),
    name: ACCEPT_ENCODING
}
