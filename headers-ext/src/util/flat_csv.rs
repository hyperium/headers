use std::fmt;
use std::iter::FromIterator;

use bytes::{Bytes, BytesMut};
use headers_core::decode::TryFromValues;
use ::HeaderValue;

// A single `HeaderValue` that can flatten multiple values with commas.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlatCsv {
    pub(crate) value: HeaderValue,
}

impl FlatCsv {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &str> {
        self
            .value
            .to_str()
            .ok()
            .into_iter()
            .map(|value_str| {
                value_str
                    .split(',')
                    .map(|item| item.trim())
            })
            .flatten()
    }
}

impl TryFromValues for FlatCsv {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        Some(values.collect())
    }
}

impl From<HeaderValue> for FlatCsv {
    fn from(value: HeaderValue) -> FlatCsv {
        FlatCsv {
            value,
        }
    }
}


impl<'a> From<&'a FlatCsv> for HeaderValue {
    fn from(flat: &'a FlatCsv) -> HeaderValue {
        flat.value.clone()
    }
}

impl fmt::Debug for FlatCsv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl<'a> FromIterator<&'a HeaderValue> for FlatCsv {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        let mut values = iter.into_iter();

        // Common case is there is only 1 value, optimize for that
        if let (1, Some(1)) = values.size_hint() {
            return values
                .next()
                .expect("size_hint claimed 1 item")
                .clone()
                .into();
        }

        // Otherwise, there are multiple, so this should merge them into 1.
        let bytes = values
            .next()
            .cloned()
            .map(Bytes::from)
            .unwrap_or_else(|| Bytes::new());

        let mut buf = BytesMut::from(bytes);

        for val in values {
            buf.extend_from_slice(b", ");
            buf.extend_from_slice(val.as_bytes());
        }

        let val = HeaderValue::from_shared(buf.freeze())
            .expect("comma separated HeaderValues are valid");

        val.into()
    }
}

// TODO: would be great if there was a way to de-dupe these with above
impl FromIterator<HeaderValue> for FlatCsv {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HeaderValue>,
    {
        let mut values = iter.into_iter();

        // Common case is there is only 1 value, optimize for that
        if let (1, Some(1)) = values.size_hint() {
            return values
                .next()
                .expect("size_hint claimed 1 item")
                .into();
        }

        // Otherwise, there are multiple, so this should merge them into 1.
        let bytes = values
            .next()
            .map(Bytes::from)
            .unwrap_or_else(|| Bytes::new());

        let mut buf = BytesMut::from(bytes);

        for val in values {
            buf.extend_from_slice(b", ");
            buf.extend_from_slice(val.as_bytes());
        }

        let val = HeaderValue::from_shared(buf.freeze())
            .expect("comma separated HeaderValues are valid");

        val.into()
    }
}

