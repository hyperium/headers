//! Utility functions for Header implementations.

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

/// Reads a comma-delimited raw header into a Vec.
#[inline]
pub fn from_comma_delimited<T, E>(values: &mut ::Values) -> Option<E>
where
    T: ::std::str::FromStr,
    E: ::std::iter::FromIterator<T>,
{
    values
        .flat_map(|value| {
            value
                .to_str()
                .into_iter()
                .flat_map(|string| {
                    string
                        .split(',')
                        .filter_map(|x| match x.trim() {
                            "" => None,
                            y => Some(y)
                        })
                        .map(|x| x.parse().map_err(|_| ()))
                })
        })
        .collect::<Result<E, ()>>()
        .ok()
}

