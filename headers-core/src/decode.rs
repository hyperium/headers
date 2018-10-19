//! Decoding utilities.

use http::header::HeaderValue;

/// A helper trait for use when deriving `Header`.
pub trait TryFromValues: Sized {
    /// Try to convert from the values into an instance of `Self`.
    fn try_from_values(values: &mut ::Values) -> Option<Self>;
}

impl TryFromValues for HeaderValue {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        values
            .next()
            .cloned()
    }
}

