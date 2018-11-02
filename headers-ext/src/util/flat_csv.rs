use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;

use bytes::{Bytes, BytesMut};
use headers_core::decode::TryFromValues;
use ::HeaderValue;

// A single `HeaderValue` that can flatten multiple values with commas.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlatCsv<Sep = Comma> {
    pub(crate) value: HeaderValue,
    _marker: PhantomData<Sep>,
}

pub(crate) trait Separator {
    const SEP: u8;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Comma {}

impl Separator for Comma {
    const SEP: u8 = b',';
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SemiColon {}

impl Separator for SemiColon {
    const SEP: u8 = b';';
}

impl<Sep: Separator> FlatCsv<Sep> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &str> {
        self
            .value
            .to_str()
            .ok()
            .into_iter()
            .flat_map(|value_str| {
                value_str
                    .split(Sep::SEP as char)
                    .map(|item| item.trim())
            })
    }
}

impl<Sep: Separator> TryFromValues for FlatCsv<Sep> {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let flat = values.collect();
        Ok(flat)
    }
}

impl<Sep> From<HeaderValue> for FlatCsv<Sep> {
    fn from(value: HeaderValue) -> Self {
        FlatCsv {
            value,
            _marker: PhantomData,
        }
    }
}


impl<'a, Sep> From<&'a FlatCsv<Sep>> for HeaderValue {
    fn from(flat: &'a FlatCsv<Sep>) -> HeaderValue {
        flat.value.clone()
    }
}

impl<Sep> fmt::Debug for FlatCsv<Sep> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl<'a, Sep: Separator> FromIterator<&'a HeaderValue> for FlatCsv<Sep> {
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
            buf.extend_from_slice(&[Sep::SEP, b' ']);
            buf.extend_from_slice(val.as_bytes());
        }

        let val = HeaderValue::from_shared(buf.freeze())
            .expect("comma separated HeaderValues are valid");

        val.into()
    }
}

// TODO: would be great if there was a way to de-dupe these with above
impl<Sep: Separator> FromIterator<HeaderValue> for FlatCsv<Sep> {
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
            buf.extend_from_slice(&[Sep::SEP, b' ']);
            buf.extend_from_slice(val.as_bytes());
        }

        let val = HeaderValue::from_shared(buf.freeze())
            .expect("comma separated HeaderValues are valid");

        val.into()
    }
}

