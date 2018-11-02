#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! dox

extern crate base64;
#[macro_use]
extern crate bitflags;
extern crate bytes;
extern crate headers_core;
#[macro_use]
extern crate headers_derive;
extern crate http;
extern crate mime;
extern crate sha1;
extern crate time;

pub use headers_core::{
    Header,
    HeaderMapExt,
};

pub use http::HeaderMap;

pub use http::header::{
    HeaderName,
    HeaderValue,
};

#[macro_use]
mod util;
mod common;

pub use self::common::*;
