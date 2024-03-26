use crate::{Header, HeaderMapExt};

/// An external trait adding helper methods to http response builder.
pub trait ResponseBuilderExt: self::sealed::Sealed {
    /// Appends a typed header to this response builder.
    fn typed_header(self, header: impl Header) -> Self;
}

impl ResponseBuilderExt for http::response::Builder {
    fn typed_header(mut self, header: impl Header) -> Self {
        self.headers_mut()
            .map(|header_map| header_map.typed_insert(header));
        self
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for http::response::Builder {}
}

#[cfg(test)]
mod tests {
    use super::ResponseBuilderExt;

    #[test]
    fn test_with_header_map_get_method() {
        let response = http::Response::builder()
            .typed_header(crate::ContentType::text())
            .body(())
            .unwrap();

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/plain",
        );
    }
}
