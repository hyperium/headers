//! Wall-clock counterparts to the Callgrind benchmarks.

use std::ops::Bound;

use test::{black_box, Bencher};

use super::{
    Age, CacheControl, Connection, ContentDisposition, ContentLength, ContentRange, Cookie, Date,
    ETag, Host, IfMatch, Range, ReferrerPolicy, StrictTransportSecurity, TransferEncoding,
};
use crate::{Header, HeaderValue};

fn vals(raw: &str) -> Vec<HeaderValue> {
    vec![HeaderValue::from_bytes(raw.as_bytes()).unwrap()]
}

fn vals2(a: &str, b: &str) -> Vec<HeaderValue> {
    vec![
        HeaderValue::from_bytes(a.as_bytes()).unwrap(),
        HeaderValue::from_bytes(b.as_bytes()).unwrap(),
    ]
}

fn cookie_values_8() -> Vec<HeaderValue> {
    [
        "a=11111111",
        "b=22222222",
        "c=33333333",
        "d=44444444",
        "e=55555555",
        "f=66666666",
        "g=77777777",
        "h=88888888",
    ]
    .iter()
    .map(|value| HeaderValue::from_static(value))
    .collect()
}

fn typed<H: Header>(raw: &str) -> H {
    H::decode(&mut vals(raw).iter()).ok().unwrap()
}

fn bench_decode<H: Header>(b: &mut Bencher, values: Vec<HeaderValue>) {
    b.bytes = values
        .iter()
        .map(|value| value.as_bytes().len() as u64)
        .sum();
    b.iter(|| {
        let mut iter = black_box(values.as_slice()).iter();
        black_box(H::decode(&mut iter).ok())
    });
}

fn bench_encode<H: Header>(b: &mut Bencher, raw: &str) {
    let header = typed::<H>(raw);
    b.bytes = raw.len() as u64;
    b.iter(|| {
        let mut values = Vec::new();
        black_box(&header).encode(&mut values);
        black_box(values)
    });
}

macro_rules! decode {
    ($name:ident, $ty:ty, $raw:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            bench_decode::<$ty>(b, vals($raw));
        }
    };
}

macro_rules! encode {
    ($name:ident, $ty:ty, $raw:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            bench_encode::<$ty>(b, $raw);
        }
    };
}

mod decode {
    use super::*;

    decode!(content_length, ContentLength, "1234567");

    #[bench]
    fn content_length_duplicates(b: &mut Bencher) {
        bench_decode::<ContentLength>(b, vals2("1234567", "1234567"));
    }

    decode!(content_length_one_digit, ContentLength, "7");
    decode!(
        content_length_nineteen_digits,
        ContentLength,
        "9999999999999999999"
    );
    decode!(
        content_length_max_u64,
        ContentLength,
        "18446744073709551615"
    );
    decode!(
        content_length_overflow,
        ContentLength,
        "18446744073709551616"
    );
    decode!(
        content_length_leading_zeroes,
        ContentLength,
        "00000000000000000000000000000000000000001"
    );
    decode!(age, Age, "3600");
    decode!(cookie_single, Cookie, "SID=31d4d96e407aad42; lang=en-US");

    #[bench]
    fn cookie_multi(b: &mut Bencher) {
        bench_decode::<Cookie>(b, vals2("SID=31d4d96e407aad42", "lang=en-US"));
    }

    #[bench]
    fn cookie_many(b: &mut Bencher) {
        bench_decode::<Cookie>(b, cookie_values_8());
    }

    decode!(
        cache_control,
        CacheControl,
        "max-age=100, private, no-cache"
    );
    decode!(
        cache_control_quoted,
        CacheControl,
        "community=\"UCI,group\", max-age=100, private"
    );
    decode!(
        cache_control_many,
        CacheControl,
        "a=1, b=2, c=3, d=4, e=5, f=6, g=7, h=8, i=9, j=10, k=11, l=12, m=13, n=14, o=15, p=16"
    );
    decode!(connection, Connection, "keep-alive");
    decode!(etag, ETag, "\"0123456789abcdef0123456789abcdef\"");
    decode!(
        etag_64,
        ETag,
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
    );
    decode!(
        etag_128,
        ETag,
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
    );
    decode!(
        etag_256,
        ETag,
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
    );
    decode!(date, Date, "Sun, 06 Nov 1994 08:49:37 GMT");
    decode!(
        strict_transport_security,
        StrictTransportSecurity,
        "max-age=31536000; includeSubdomains"
    );
    decode!(
        strict_transport_security_quoted,
        StrictTransportSecurity,
        " MAX-AGE = \"31536000\" ; includeSubDomains "
    );
    decode!(host, Host, "example.com:8080");
    decode!(range, Range, "bytes=0-1023,2048-4095,8192-,-4096");
    decode!(content_range, ContentRange, "bytes 12345-67890/1000000");
    decode!(
        referrer_policy,
        ReferrerPolicy,
        "unknown, no-referrer, same-origin, origin-when-cross-origin, strict-origin-when-cross-origin"
    );
}

mod encode {
    use super::*;

    encode!(content_length, ContentLength, "1234567");
    encode!(age, Age, "3600");
    encode!(cookie, Cookie, "SID=31d4d96e407aad42; lang=en-US");
    encode!(
        cache_control,
        CacheControl,
        "max-age=100, private, no-cache"
    );
    encode!(
        cache_control_all_directives,
        CacheControl,
        "no-cache, no-store, no-transform, only-if-cached, must-revalidate, must-understand, public, private, immutable, proxy-revalidate, max-age=100, max-stale=200, min-fresh=300, s-maxage=400"
    );
    encode!(connection, Connection, "keep-alive");
    encode!(etag, ETag, "\"0123456789abcdef0123456789abcdef\"");
    encode!(date, Date, "Sun, 06 Nov 1994 08:49:37 GMT");
    encode!(
        strict_transport_security,
        StrictTransportSecurity,
        "max-age=31536000; includeSubdomains"
    );
    encode!(host, Host, "example.com:8080");
    encode!(range, Range, "bytes=0-1023,2048-4095,8192-,-4096");
    encode!(content_range, ContentRange, "bytes 12345-67890/1000000");
}

mod query {
    use super::*;

    #[bench]
    fn transfer_encoding_is_chunked_single(b: &mut Bencher) {
        let header = typed::<TransferEncoding>("chunked");
        b.bytes = "chunked".len() as u64;
        b.iter(|| black_box(&header).is_chunked());
    }

    #[bench]
    fn transfer_encoding_is_chunked_multiple(b: &mut Bencher) {
        let raw = "gzip, deflate, br, chunked";
        let header = typed::<TransferEncoding>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).is_chunked());
    }

    #[bench]
    fn etag_is_weak_strong(b: &mut Bencher) {
        let raw = "\"0123456789abcdef0123456789abcdef\"";
        let header = typed::<ETag>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).is_weak());
    }

    #[bench]
    fn etag_is_weak_weak(b: &mut Bencher) {
        let raw = "W/\"0123456789abcdef0123456789abcdef\"";
        let header = typed::<ETag>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).is_weak());
    }

    #[bench]
    fn if_match_precondition_passes(b: &mut Bencher) {
        let raw = "\"0123456789abcdef0123456789abcdef\"";
        let etag = typed::<ETag>(raw);
        let condition = IfMatch::from(etag.clone());
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&condition).precondition_passes(black_box(&etag)));
    }

    #[bench]
    fn if_match_list_precondition_passes(b: &mut Bencher) {
        let list = "\"first\", \"second\", \"0123456789abcdef0123456789abcdef\"";
        let target = "\"0123456789abcdef0123456789abcdef\"";
        let condition = typed::<IfMatch>(list);
        let etag = typed::<ETag>(target);
        b.bytes = list.len() as u64;
        b.iter(|| black_box(&condition).precondition_passes(black_box(&etag)));
    }

    #[bench]
    fn range_satisfiable_ranges(b: &mut Bencher) {
        let raw = "bytes=0-1023,2048-4095,8192-,-4096";
        let header = typed::<Range>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| {
            black_box(&header)
                .satisfiable_ranges(black_box(16384))
                .collect::<Vec<(Bound<u64>, Bound<u64>)>>()
        });
    }

    #[bench]
    fn content_disposition_is_attachment(b: &mut Bencher) {
        let raw =
            "attachment; filename=\"a-very-long-file-name-used-to-exercise-prefix-parsing.tar.gz\"";
        let header = typed::<ContentDisposition>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).is_attachment());
    }

    #[bench]
    fn construct_range_bytes(b: &mut Bencher) {
        b.iter(|| black_box(Range::bytes(black_box(0)..=black_box(1023)).unwrap()));
    }

    #[bench]
    fn cookie_get_last(b: &mut Bencher) {
        let raw = "a=1; b=2; c=3; d=4; e=5; target=abcdefgh";
        let header = typed::<Cookie>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).get(black_box("target")) == Some("abcdefgh"));
    }

    #[bench]
    fn cookie_len(b: &mut Bencher) {
        let raw = "a=1; b=2; c=3; d=4; e=5; target=abcdefgh";
        let header = typed::<Cookie>(raw);
        b.bytes = raw.len() as u64;
        b.iter(|| black_box(&header).len());
    }
}
