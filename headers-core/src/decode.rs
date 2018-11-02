//! Decoding utilities.

use http::header::HeaderValue;

/// A helper trait for use when deriving `Header`.
pub trait TryFromValues: Sized {
    /// Try to convert from the values into an instance of `Self`.
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>;
}

impl TryFromValues for HeaderValue {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .cloned()
            .ok_or_else(|| ::Error::invalid())
    }
}

