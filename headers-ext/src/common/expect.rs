/// The `Expect` header.
///
/// > The "Expect" header field in a request indicates a certain set of
/// > behaviors (expectations) that need to be supported by the server in
/// > order to properly handle this request.  The only such expectation
/// > defined by this specification is 100-continue.
/// >
/// >    Expect  = "100-continue"
///
/// # Example
///
/// ```
/// use headers::Expect;
///
/// let expect = Expect::CONTINUE;
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Expect(());

impl Expect {
    /// "100-continue"
    pub const CONTINUE: Expect = Expect(());
}

impl ::Header for Expect {
    const NAME: &'static ::HeaderName = &::http::header::EXPECT;

    fn decode(values: &mut ::Values) -> ::Result<Expect> {
        if values.next_or_empty()? == "100-continue" {
            Ok(Expect::CONTINUE)
        } else {
            Err(::Error::invalid())
        }
    }

    fn encode(&self, values: &mut ::ToValues) {
        values.append(::HeaderValue::from_static("100-continue"));
    }
}
