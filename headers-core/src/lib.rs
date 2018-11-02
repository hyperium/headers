#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! # headers-core
//!
//! This is the core crate of the typed HTTP headers system, providing only
//! the relevant traits. All actual header implementations are in other crates.

extern crate bytes;
extern crate http;

pub use http::header::{self, HeaderName, HeaderValue};

pub mod decode;
//pub mod encode;

/// A trait for any object that will represent a header field and value.
///
/// This trait represents the construction and identification of headers,
/// and contains trait-object unsafe methods.
pub trait Header {
    /// The name of this header.
    const NAME: &'static HeaderName;

    /// Decode this type from an iterator of `HeaderValue`s.
    fn decode<'i, I>(values: &mut I) -> Option<Self>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>;

    /// Encode this type to a `HeaderMap`.
    ///
    /// This function should be infallible. Any errors converting to a
    /// `HeaderValue` should have been caught when parsing or constructing
    /// this value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E);
}

/// A builder to append `HeaderValue`s to during `Header::encode`.
#[derive(Debug)]
struct ToValues<'a> {
    state: State<'a>,
}

#[derive(Debug)]
enum State<'a> {
    First(http::header::Entry<'a, HeaderValue>),
    Latter(http::header::OccupiedEntry<'a, HeaderValue>),
    Tmp,
}

impl<'a> Extend<HeaderValue> for ToValues<'a> {
    fn extend<T: IntoIterator<Item=HeaderValue>>(&mut self, iter: T) {
        for value in iter {
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
}

/// An extension trait adding "typed" methods to `http::HeaderMap`.
pub trait HeaderMapExt: self::sealed::Sealed {
    /// Inserts the typed `Header` into this `HeaderMap`.
    fn typed_insert<H>(&mut self, header: H)
    where
        H: Header;

    /// Tries to find the header by name, and then decode it into `H`.
    fn typed_get<H>(&self) -> Option<H>
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

    fn typed_get<H>(&self) -> Option<H>
    where
        H: Header,
    {
        let mut values = self.get_all(H::NAME).iter();
        H::decode(&mut values)
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for ::http::HeaderMap {}
}
