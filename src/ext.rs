use crate::{Header, HeaderValue};

/// An external trait adding helper methods to types which implement [`Header`] trait.
pub trait HeaderExt: Header + self::sealed::Sealed {
    /// Encode this [`Header`] to [`HeaderValue`].
    fn encode_to_value(&self) -> HeaderValue;
}

impl<H> HeaderExt for H
where
    H: Header + self::sealed::Sealed,
{
    fn encode_to_value(&self) -> HeaderValue {
        let mut container = Vec::with_capacity(1);
        self.encode(&mut container);
        container.remove(0)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<H: crate::Header> Sealed for H {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_header() {
        let header_value = crate::AcceptRanges::bytes().encode_to_value();
        assert_eq!(header_value, HeaderValue::from_static("bytes"));
    }
}
