use crate::{Header, HeaderMapExt};

/// An external trait adding helper methods to http request builder.
pub trait RequestBuilderExt: self::sealed::Sealed {
    /// Appends a typed header to this request builder.
    fn typed_header(self, header: impl Header) -> Self;
}

impl RequestBuilderExt for http::request::Builder {
    fn typed_header(mut self, header: impl Header) -> Self {
        self.headers_mut()
            .map(|header_map| header_map.typed_insert(header));
        self
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for http::request::Builder {}
}

#[cfg(test)]
mod tests {
    use super::RequestBuilderExt;

    #[test]
    fn test_with_header_map_get_method() {
        let request = http::Request::builder()
            .typed_header(crate::ContentType::text())
            .body(())
            .unwrap();

        assert_eq!(
            request.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/plain",
        );
    }
}
