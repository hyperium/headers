extern crate http;

pub use http::header::{self, HeaderName, HeaderValue};

mod error;
pub mod parsing;

pub use self::error::{Error, Result};

/// A trait for any object that will represent a header field and value.
///
/// This trait represents the construction and identification of headers,
/// and contains trait-object unsafe methods.
pub trait Header {
    /// The name of this header.
    const NAME: &'static HeaderName;

    /// Decode this type from a `HeaderValue`.
    fn decode(values: &mut Values) -> Result<Self>
    where
        Self: Sized;

    /// Encode this type to a `HeaderMap`.
    ///
    /// This function should be infallible. Any errors converting to a
    /// `HeaderValue` should have been caught when parsing or constructing
    /// this value.
    fn encode(&self, values: &mut ToValues);
}

#[derive(Debug)]
pub struct Values<'a> {
    inner: http::header::ValueIter<'a, http::header::HeaderValue>,
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a HeaderValue;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub struct ToValues<'a> {
    state: State<'a>,
}

enum State<'a> {
    First(http::header::Entry<'a, HeaderValue>),
    Latter(http::header::OccupiedEntry<'a, HeaderValue>),
    Tmp,
}

impl<'a> ToValues<'a> {
    pub fn append(&mut self, value: HeaderValue) {
        let entry = match ::std::mem::replace(&mut self.state, State::Tmp) {
            State::First(http::header::Entry::Occupied(mut e)) => {
                e.insert(value);
                e
            },
            State::First(http::header::Entry::Vacant(e)) => e.insert_entry(value),
            State::Latter(mut e) => {
                e.append(value);
                e
            },
            State::Tmp => unreachable!("ToValues State::Tmp"),
        };
        self.state = State::Latter(entry);
    }
}

pub trait HeaderMapExt: self::sealed::Sealed {
    fn typed_insert<H>(&mut self, header: H)
    where
        H: Header;
}

impl HeaderMapExt for http::HeaderMap {
    fn typed_insert<H>(&mut self, header: H)
    where
        H: Header,
    {
        let entry = self
            .entry(H::NAME)
            .expect("HeaderName is always valid");
        let mut values = ToValues {
            state: State::First(entry),
        };
        header.encode(&mut values);
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for ::http::HeaderMap {}
}
