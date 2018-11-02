#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! # headers-core
//!
//! This is the core crate of the typed HTTP headers system, providing only
//! the relevant traits. All actual header implementations are in other crates.

extern crate bytes;
extern crate http;

use std::fmt;

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

    /// Decode this type from a `HeaderValue`.
    fn decode<'i, I>(values: &mut I) -> Option<Self>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>;

    /// Encode this type to a `HeaderMap`.
    ///
    /// This function should be infallible. Any errors converting to a
    /// `HeaderValue` should have been caught when parsing or constructing
    /// this value.
    fn encode(&self, values: &mut ToValues);
}

/// A builder to append `HeaderValue`s to during `Header::encode`.
#[derive(Debug)]
pub struct ToValues<'a> {
    state: State<'a>,
}

#[derive(Debug)]
enum State<'a> {
    First(http::header::Entry<'a, HeaderValue>),
    Latter(http::header::OccupiedEntry<'a, HeaderValue>),
    Tmp,
}

impl<'a> ToValues<'a> {
    /// Append the `HeaderValue` to the existing list of headers.
    ///
    /// While this can be called multiple times, *most* headers should only
    /// call this once. The exceptions are outliers like `Set-Cookie`.
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

    /// Append the `impl Display` to the list of headers.
    ///
    /// # Panics
    ///
    /// Encoding `HeaderValue`s is expected to be infallible. However, not
    /// all UTF-8 sequences are valid for a `HeaderValue`. The type passed
    /// here must ensure that its resulting string is a valid `HeaderValue`.
    pub fn append_fmt<T: fmt::Display>(&mut self, fmt: T) {
        let s = fmt.to_string();
        let value = match HeaderValue::from_shared(s.into()) {
            Ok(val) => val,
            Err(err) => panic!("illegal HeaderValue; error = {:?}, fmt = \"{}\"", err, fmt),
        };
        self.append(value);
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
