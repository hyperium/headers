use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

use bytes::Bytes;
use http::header::HeaderValue;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommaDelimited<T>(T);

impl<T> CommaDelimited<T> {
    pub fn new(collection: T) -> Self {
        CommaDelimited(collection)
    }
}

impl<T> super::decode::TryFromValues for CommaDelimited<T>
where
    T: FromIterator<<T as Iterator>::Item> + Iterator,
    <T as Iterator>::Item: FromStr,
{
    fn try_from_values(values: &mut ::Values) -> ::Result<Self> {
        super::decode::from_comma_delimited(values)
            .map(CommaDelimited)
    }
}

impl<T, V> From<CommaDelimited<T>> for HeaderValue
where
    for<'a> &'a T: IntoIterator<Item=V>,
    V: fmt::Display,
{
    fn from(csv: CommaDelimited<T>) -> HeaderValue {
        // TODO: could be just BytesMut instead of String
        let s = csv.to_string();
        let bytes = Bytes::from(s);
        match HeaderValue::from_shared(bytes) {
            Ok(v) => v,
            Err(err) => panic!("illegal HeaderValue; error = {:?}, fmt = \"{}\"", err, csv),
        }
    }
}


impl<T, V> fmt::Display for CommaDelimited<T>
where
    for<'a> &'a T: IntoIterator<Item=V>,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.0.into_iter();
        if let Some(part) = iter.next() {
            fmt::Display::fmt(&part, f)?;
        }
        for part in iter {
            f.write_str(", ")?;
            fmt::Display::fmt(&part, f)?;
        }
        Ok(())
    }
}

impl<T: fmt::Debug> fmt::Debug for CommaDelimited<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

