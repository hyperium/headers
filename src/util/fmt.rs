use std::convert::TryFrom;
use std::fmt::{self, Display};

use http::HeaderValue;

pub(crate) fn fmt<T: Display>(fmt: T) -> HeaderValue {
    let mut s = String::with_capacity(64);
    fmt::write(&mut s, format_args!("{}", fmt)).expect("writing to a String cannot fail");
    match HeaderValue::try_from(s) {
        Ok(val) => val,
        Err(err) => panic!("illegal HeaderValue; error = {:?}, fmt = \"{}\"", err, fmt),
    }
}
