//! Headers container, and common header fields.
//!
//! hyper has the opinion that Headers should be strongly-typed, because that's
//! why we're using Rust in the first place. To set or get any header, an object
//! must implement the `Header` trait from this module. Several common headers
//! are already provided, such as `Host`, `ContentType`, `UserAgent`, and others.
//!
//! # Why Typed?
//!
//! Or, why not stringly-typed? Types give the following advantages:
//!
//! - More difficult to typo, since typos in types should be caught by the compiler
//! - Parsing to a proper type by default
//!
//! # Defining Custom Headers
//!
//! Hyper provides many of the most commonly used headers in HTTP. If
//! you need to define a custom header, it's easy to do while still taking
//! advantage of the type system. Hyper includes a `header!` macro for defining
//! many wrapper-style headers.
//!
//! ```
//! #[macro_use] extern crate headers;
//! use headers::Headers;
//!
//! #[derive(Header)]
//! #[header(name = "x-request-guid")]
//! struct XRequestGuid(String);
//!
//! fn main () {
//!     let mut headers = Headers::new();
//!
//!     headers.set(XRequestGuid("a proper guid".to_owned()))
//! }
//! ```
//!
//! This works well for simple "string" headers.  If you need more control,
//! you can implement the trait directly.
//!
//! ## Implementing the `Header` trait
//!
//! Consider a Do Not Track header. It can be true or false, but it represents
//! that via the numerals `1` and `0`.
//!
//! ```
//! use std::fmt;
//! use headers::{self, Formatter, Header, Raw};
//!
//! #[derive(Debug, Clone, Copy)]
//! struct Dnt(bool);
//!
//! impl Header for Dnt {
//!     fn header_name() -> &'static str {
//!         "DNT"
//!     }
//!
//!     fn parse_header(raw: &Raw) -> headers::Result<Dnt> {
//!         if raw.len() == 1 {
//!             let line = &raw[0];
//!             if line.len() == 1 {
//!                 let byte = line[0];
//!                 match byte {
//!                     b'0' => return Ok(Dnt(true)),
//!                     b'1' => return Ok(Dnt(false)),
//!                     _ => ()
//!                 }
//!             }
//!         }
//!         Err(headers::Error::Header)
//!     }
//!
//!     fn fmt_header(&self, f: &mut Formatter) -> fmt::Result {
//!         let value = if self.0 {
//!             "1"
//!         } else {
//!             "0"
//!         };
//!         f.fmt_line(&value)
//!     }
//! }
//! ```
extern crate base64;
extern crate bytes;
extern crate http;
extern crate httparse;
extern crate language_tags;
extern crate mime;
#[macro_use]
extern crate percent_encoding;
extern crate unicase;
extern crate time;

pub use http::header::{self, HeaderName, HeaderValue};

pub use self::error::{Error, Result};
pub use self::shared::*;
pub use self::common::*;
//use bytes::Bytes;

mod common;
mod error;
mod shared;
pub mod parsing;


/// A trait for any object that will represent a header field and value.
///
/// This trait represents the construction and identification of headers,
/// and contains trait-object unsafe methods.
pub trait Header {
    /// Returns the name of the header field this belongs to.
    ///
    /// This will become an associated constant once available.
    const NAME: &'static HeaderName;
    //fn name() -> &'static HeaderName;

    fn decode(values: &mut ValueIter) -> Result<Self>
    where
        Self: Sized;
    fn encode(&self, values: &mut ToValues);
    /*
    /// Parse a header from a raw stream of bytes.
    ///
    /// It's possible that a request can include a header field more than once,
    /// and in that case, the slice will have a length greater than 1. However,
    /// it's not necessarily the case that a Header is *allowed* to have more
    /// than one field value. If that's the case, you **should** return `None`
    /// if `raw.len() > 1`.
    fn parse_header(raw: &Raw) -> ::Result<Self> where Self: Sized;
    /// Format a header to outgoing stream.
    ///
    /// Most headers should be formatted on one line, and so a common pattern
    /// would be to implement `std::fmt::Display` for this type as well, and
    /// then just call `f.fmt_line(self)`.
    ///
    /// ## Note
    ///
    /// This has the ability to format a header over multiple lines.
    ///
    /// The main example here is `Set-Cookie`, which requires that every
    /// cookie being set be specified in a separate line. Almost every other
    /// case should only format as 1 single line.
    #[inline]
    fn fmt_header(&self, f: &mut Formatter) -> fmt::Result;
    */
}

pub struct ToValues;
pub type ValueIter<'a> = http::header::ValueIter<'a, http::header::HeaderValue>;

/*
#[cfg(test)]
mod tests {
    use std::fmt;
    use super::{Headers, Header, Raw, ContentLength, ContentType, Host, SetCookie};

    #[cfg(feature = "nightly")]
    use test::Bencher;

    macro_rules! make_header {
        ($name:expr, $value:expr) => ({
            let mut headers = Headers::new();
            headers.set_raw(String::from_utf8($name.to_vec()).unwrap(), $value.to_vec());
            headers
        });
        ($text:expr) => ({
            let bytes = $text;
            let colon = bytes.iter().position(|&x| x == b':').unwrap();
            make_header!(&bytes[..colon], &bytes[colon + 2..])
        })
    }
    #[test]
    fn test_from_raw() {
        let headers = make_header!(b"Content-Length", b"10");
        assert_eq!(headers.get(), Some(&ContentLength(10)));
    }

    #[derive(Clone, PartialEq, Debug)]
    struct CrazyLength(Option<bool>, usize);

    impl Header for CrazyLength {
        fn header_name() -> &'static str {
            "content-length"
        }
        fn parse_header(raw: &Raw) -> ::Result<CrazyLength> {
            use std::str::from_utf8;
            use std::str::FromStr;

            if let Some(line) = raw.one() {
                let s = try!(from_utf8(line).map(|s| FromStr::from_str(s).map_err(|_| ::Error::Header)));
                s.map(|u| CrazyLength(Some(false), u))
            } else {
                Err(::Error::Header)
            }
        }

        fn fmt_header(&self, f: &mut super::Formatter) -> fmt::Result {
            f.fmt_line(self)
        }
    }

    impl fmt::Display for CrazyLength {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let CrazyLength(ref opt, ref value) = *self;
            write!(f, "{:?}, {:?}", opt, value)
        }
    }

    #[test]
    fn test_different_structs_for_same_header() {
        let headers = make_header!(b"Content-Length: 10");
        assert_eq!(headers.get::<ContentLength>(), Some(&ContentLength(10)));
        assert_eq!(headers.get::<CrazyLength>(), Some(&CrazyLength(Some(false), 10)));
    }

    #[test]
    fn test_trailing_whitespace() {
        let headers = make_header!(b"Content-Length: 10   ");
        assert_eq!(headers.get::<ContentLength>(), Some(&ContentLength(10)));
    }

    #[test]
    fn test_multiple_reads() {
        let headers = make_header!(b"Content-Length: 10");
        let ContentLength(one) = *headers.get::<ContentLength>().unwrap();
        let ContentLength(two) = *headers.get::<ContentLength>().unwrap();
        assert_eq!(one, two);
    }

    #[test]
    fn test_different_reads() {
        let mut headers = Headers::new();
        headers.set_raw("Content-Length", "10");
        headers.set_raw("Content-Type", "text/plain");
        let ContentLength(_) = *headers.get::<ContentLength>().unwrap();
        let ContentType(_) = *headers.get::<ContentType>().unwrap();
    }

    #[test]
    fn test_typed_get_raw() {
        let mut headers = Headers::new();
        headers.set(ContentLength(15));
        assert_eq!(headers.get_raw("content-length").unwrap(), "15");

        headers.set(SetCookie(vec![
            "foo=bar".to_string(),
            "baz=quux; Path=/path".to_string()
        ]));
        assert_eq!(headers.get_raw("set-cookie").unwrap(), &["foo=bar", "baz=quux; Path=/path"][..]);
    }

    #[test]
    fn test_get_mutable() {
        let mut headers = make_header!(b"Content-Length: 10");
        *headers.get_mut::<ContentLength>().unwrap() = ContentLength(20);
        assert_eq!(headers.get_raw("content-length").unwrap(), &[b"20".to_vec()][..]);
        assert_eq!(*headers.get::<ContentLength>().unwrap(), ContentLength(20));
    }

    #[test]
    fn test_headers_to_string() {
        let mut headers = Headers::new();
        headers.set(ContentLength(15));
        headers.set(Host::new("foo.bar", None));

        let s = headers.to_string();
        assert!(s.contains("Host: foo.bar\r\n"));
        assert!(s.contains("Content-Length: 15\r\n"));
    }

    #[test]
    fn test_headers_to_string_raw() {
        let mut headers = make_header!(b"Content-Length: 10");
        headers.set_raw("x-foo", vec![b"foo".to_vec(), b"bar".to_vec()]);
        let s = headers.to_string();
        assert_eq!(s, "Content-Length: 10\r\nx-foo: foo\r\nx-foo: bar\r\n");
    }

    #[test]
    fn test_set_raw() {
        let mut headers = Headers::new();
        headers.set(ContentLength(10));
        headers.set_raw("content-LENGTH", vec![b"20".to_vec()]);
        assert_eq!(headers.get_raw("Content-length").unwrap(), &[b"20".to_vec()][..]);
        assert_eq!(headers.get(), Some(&ContentLength(20)));
    }

    #[test]
    fn test_append_raw() {
        let mut headers = Headers::new();
        headers.set(ContentLength(10));
        headers.append_raw("content-LENGTH", b"20".to_vec());
        assert_eq!(headers.get_raw("Content-length").unwrap(), &[b"10".to_vec(), b"20".to_vec()][..]);
        headers.append_raw("x-foo", "bar");
        assert_eq!(headers.get_raw("x-foo").unwrap(), &[b"bar".to_vec()][..]);
    }

    #[test]
    fn test_remove_raw() {
        let mut headers = Headers::new();
        headers.set_raw("content-LENGTH", vec![b"20".to_vec()]);
        headers.remove_raw("content-LENGTH");
        assert_eq!(headers.get_raw("Content-length"), None);
    }

    #[test]
    fn test_remove() {
        let mut headers = Headers::new();
        headers.set(ContentLength(10));
        assert_eq!(headers.remove(), Some(ContentLength(10)));
        assert_eq!(headers.len(), 0);

        headers.set(ContentLength(9));
        assert_eq!(headers.len(), 1);
        assert!(headers.remove::<CrazyLength>().is_none());
        assert_eq!(headers.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut headers = Headers::new();
        headers.set(ContentLength(10));
        assert_eq!(headers.len(), 1);
        headers.set(ContentType::json());
        assert_eq!(headers.len(), 2);
        // Redundant, should not increase count.
        headers.set(ContentLength(20));
        assert_eq!(headers.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut headers = Headers::new();
        headers.set(ContentLength(10));
        headers.set(ContentType::json());
        assert_eq!(headers.len(), 2);
        headers.clear();
        assert_eq!(headers.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        for header in headers.iter() {
            assert!(header.is::<ContentLength>());
            assert_eq!(header.name(), <ContentLength as Header>::header_name());
            assert_eq!(header.value(), Some(&ContentLength(11)));
            assert_eq!(header.value_string(), "11".to_owned());
        }
    }

    #[test]
    fn test_header_view_value_string() {
        let mut headers = Headers::new();
        headers.set_raw("foo", vec![b"one".to_vec(), b"two".to_vec()]);
        for header in headers.iter() {
            assert_eq!(header.name(), "foo");
            assert_eq!(header.value_string(), "one, two");
        }
    }

    #[test]
    fn test_header_view_raw() {
        let mut headers = Headers::new();
        headers.set_raw("foo", vec![b"one".to_vec(), b"two".to_vec()]);
        for header in headers.iter() {
            assert_eq!(header.name(), "foo");
            let values: Vec<&[u8]> = header.raw().iter().collect();
            assert_eq!(values, vec![b"one", b"two"]);
        }
    }

    #[test]
    fn test_eq() {
        let mut headers1 = Headers::new();
        let mut headers2 = Headers::new();

        assert_eq!(headers1, headers2);

        headers1.set(ContentLength(11));
        headers2.set(Host::new("foo.bar", None));
        assert_ne!(headers1, headers2);

        headers1 = Headers::new();
        headers2 = Headers::new();

        headers1.set(ContentLength(11));
        headers2.set(ContentLength(11));
        assert_eq!(headers1, headers2);

        headers1.set(ContentLength(10));
        assert_ne!(headers1, headers2);

        headers1 = Headers::new();
        headers2 = Headers::new();

        headers1.set(Host::new("foo.bar", None));
        headers1.set(ContentLength(11));
        headers2.set(ContentLength(11));
        assert_ne!(headers1, headers2);
    }

    #[test]
    #[cfg(feature = "compat")]
    fn test_compat() {
        use http;

        let mut orig_hyper_headers = Headers::new();
        orig_hyper_headers.set(ContentLength(11));
        orig_hyper_headers.set(Host::new("foo.bar", None));
        orig_hyper_headers.append_raw("x-foo", b"bar".to_vec());
        orig_hyper_headers.append_raw("x-foo", b"quux".to_vec());

        let mut orig_http_headers = http::HeaderMap::new();
        orig_http_headers.insert(http::CONTENT_LENGTH, "11".parse().unwrap());
        orig_http_headers.insert(http::HOST, "foo.bar".parse().unwrap());
        orig_http_headers.append("x-foo", "bar".parse().unwrap());
        orig_http_headers.append("x-foo", "quux".parse().unwrap());

        let conv_hyper_headers: Headers = orig_http_headers.clone().into();
        let conv_http_headers: http::HeaderMap = orig_hyper_headers.clone().into();
        assert_eq!(orig_hyper_headers, conv_hyper_headers);
        assert_eq!(orig_http_headers, conv_http_headers);
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_new(b: &mut Bencher) {
        b.iter(|| {
            let mut h = Headers::new();
            h.set(ContentLength(11));
            h
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_get(b: &mut Bencher) {
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        b.iter(|| assert_eq!(headers.get::<ContentLength>(), Some(&ContentLength(11))))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_get_miss(b: &mut Bencher) {
        let headers = Headers::new();
        b.iter(|| assert!(headers.get::<ContentLength>().is_none()))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_get_miss_previous_10(b: &mut Bencher) {
        let mut headers = Headers::new();
        for i in 0..10 {
            headers.set_raw(format!("non-standard-{}", i), "hi");
        }
        b.iter(|| assert!(headers.get::<ContentLength>().is_none()))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_set(b: &mut Bencher) {
        let mut headers = Headers::new();
        b.iter(|| headers.set(ContentLength(12)))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_set_previous_10(b: &mut Bencher) {
        let mut headers = Headers::new();
        for i in 0..10 {
            headers.set_raw(format!("non-standard-{}", i), "hi");
        }
        b.iter(|| headers.set(ContentLength(12)))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_set_raw(b: &mut Bencher) {
        let mut headers = Headers::new();
        b.iter(|| headers.set_raw("non-standard", "hello"))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_set_raw_previous_10(b: &mut Bencher) {
        let mut headers = Headers::new();
        for i in 0..10 {
            headers.set_raw(format!("non-standard-{}", i), "hi");
        }
        b.iter(|| headers.set_raw("non-standard", "hello"))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_has(b: &mut Bencher) {
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        b.iter(|| assert!(headers.has::<ContentLength>()))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_view_is(b: &mut Bencher) {
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        let mut iter = headers.iter();
        let view = iter.next().unwrap();
        b.iter(|| assert!(view.is::<ContentLength>()))
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_headers_fmt(b: &mut Bencher) {
        use std::fmt::Write;
        let mut buf = String::with_capacity(64);
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        headers.set(ContentType::json());
        b.bytes = headers.to_string().len() as u64;
        b.iter(|| {
            let _ = write!(buf, "{}", headers);
            ::test::black_box(&buf);
            unsafe { buf.as_mut_vec().set_len(0); }
        })
    }
}
*/
