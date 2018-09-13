use std::fmt;

use bytes::{Bytes, BytesMut};
use headers_core::decode::TryFromValues;
use ::HeaderValue;

// A single `HeaderValue` that can flatten multiple values with commas.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlatCsv {
    pub(crate) value: HeaderValue,
}

impl TryFromValues for FlatCsv {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        // Common case is there is only 1 value, optimize for that
        if let (1, Some(1)) = values.size_hint() {
            return values
                .next()
                .cloned()
                .map(FlatCsv::from)
        }

        // Otherwise, there are multiple, so this should merge them into 1.
        let bytes: Bytes = values
            .next()?
            .clone()
            .into();

        let mut buf = BytesMut::from(bytes);

        for val in values {
            buf.extend_from_slice(b", ");
            buf.extend_from_slice(val.as_bytes());
        }

        let val = HeaderValue::from_shared(buf.freeze())
            .expect("comma separated HeaderValues are valid");

        Some(val.into())
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

