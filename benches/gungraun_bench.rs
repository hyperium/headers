//! Callgrind benchmarks for header parsing, encoding, and queries.

use gungraun::prelude::*;
use std::hint::black_box;

use headers::{
    Age, CacheControl, Connection, ContentDisposition, ContentLength, ContentRange, Cookie, Date,
    ETag, Header, HeaderValue, Host, IfMatch, Range, ReferrerPolicy, StrictTransportSecurity,
    TransferEncoding,
};

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

#[library_benchmark(setup = vals)]
#[bench::content_length("1234567")]
#[bench::duplicates(args = ("1234567", "1234567"), setup = vals2)]
#[bench::one_digit("7")]
#[bench::nineteen_digits("9999999999999999999")]
#[bench::max_u64("18446744073709551615")]
#[bench::overflow("18446744073709551616")]
#[bench::leading_zeroes("00000000000000000000000000000000000000001")]
fn decode_content_length(values: Vec<HeaderValue>) -> Option<ContentLength> {
    black_box(ContentLength::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::age("3600")]
fn decode_age(values: Vec<HeaderValue>) -> Option<Age> {
    black_box(Age::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark]
#[bench::single(args = ("SID=31d4d96e407aad42; lang=en-US"), setup = vals)]
#[bench::multi(args = ("SID=31d4d96e407aad42", "lang=en-US"), setup = vals2)]
#[bench::many(setup = cookie_values_8)]
fn decode_cookie(values: Vec<HeaderValue>) -> Option<Cookie> {
    black_box(Cookie::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::cache_control("max-age=100, private, no-cache")]
#[bench::quoted("community=\"UCI,group\", max-age=100, private")]
fn decode_cache_control(values: Vec<HeaderValue>) -> Option<CacheControl> {
    black_box(CacheControl::decode(&mut black_box(values).iter()).ok())
}

// Unknown directives keep the benchmark focused on CSV splitting.
#[library_benchmark(setup = vals)]
#[bench::cache_control_many(
    "a=1, b=2, c=3, d=4, e=5, f=6, g=7, h=8, i=9, j=10, k=11, l=12, m=13, n=14, o=15, p=16"
)]
fn decode_cache_control_many(values: Vec<HeaderValue>) -> Option<CacheControl> {
    black_box(CacheControl::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::connection("keep-alive")]
fn decode_connection(values: Vec<HeaderValue>) -> Option<Connection> {
    black_box(Connection::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::etag("\"0123456789abcdef0123456789abcdef\"")]
#[bench::bytes_64("\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"")]
#[bench::bytes_128(
    "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
)]
#[bench::long(
    "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
)]
fn decode_etag(values: Vec<HeaderValue>) -> Option<ETag> {
    black_box(ETag::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::date("Sun, 06 Nov 1994 08:49:37 GMT")]
fn decode_date(values: Vec<HeaderValue>) -> Option<Date> {
    black_box(Date::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::sts("max-age=31536000; includeSubdomains")]
#[bench::quoted(" MAX-AGE = \"31536000\" ; includeSubDomains ")]
fn decode_sts(values: Vec<HeaderValue>) -> Option<StrictTransportSecurity> {
    black_box(StrictTransportSecurity::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::host("example.com:8080")]
fn decode_host(values: Vec<HeaderValue>) -> Option<Host> {
    black_box(Host::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::range("bytes=0-1023,2048-4095,8192-,-4096")]
fn decode_range(values: Vec<HeaderValue>) -> Option<Range> {
    black_box(Range::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::content_range("bytes 12345-67890/1000000")]
fn decode_content_range(values: Vec<HeaderValue>) -> Option<ContentRange> {
    black_box(ContentRange::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = vals)]
#[bench::referrer_policy(
    "unknown, no-referrer, same-origin, origin-when-cross-origin, strict-origin-when-cross-origin"
)]
fn decode_referrer_policy(values: Vec<HeaderValue>) -> Option<ReferrerPolicy> {
    black_box(ReferrerPolicy::decode(&mut black_box(values).iter()).ok())
}

#[library_benchmark(setup = typed::<ContentLength>)]
#[bench::content_length("1234567")]
fn encode_content_length(header: ContentLength) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Age>)]
#[bench::age("3600")]
fn encode_age(header: Age) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Cookie>)]
#[bench::cookie("SID=31d4d96e407aad42; lang=en-US")]
fn encode_cookie(header: Cookie) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<CacheControl>)]
#[bench::cache_control("max-age=100, private, no-cache")]
#[bench::all_directives(
    "no-cache, no-store, no-transform, only-if-cached, must-revalidate, must-understand, public, private, immutable, proxy-revalidate, max-age=100, max-stale=200, min-fresh=300, s-maxage=400"
)]
fn encode_cache_control(header: CacheControl) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Connection>)]
#[bench::connection("keep-alive")]
fn encode_connection(header: Connection) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<ETag>)]
#[bench::etag("\"0123456789abcdef0123456789abcdef\"")]
fn encode_etag(header: ETag) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Date>)]
#[bench::date("Sun, 06 Nov 1994 08:49:37 GMT")]
fn encode_date(header: Date) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<StrictTransportSecurity>)]
#[bench::sts("max-age=31536000; includeSubdomains")]
fn encode_sts(header: StrictTransportSecurity) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Host>)]
#[bench::host("example.com:8080")]
fn encode_host(header: Host) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<Range>)]
#[bench::range("bytes=0-1023,2048-4095,8192-,-4096")]
fn encode_range(header: Range) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<ContentRange>)]
#[bench::content_range("bytes 12345-67890/1000000")]
fn encode_content_range(header: ContentRange) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    black_box(&header).encode(&mut out);
    black_box(out)
}

#[library_benchmark(setup = typed::<TransferEncoding>)]
#[bench::single("chunked")]
#[bench::multiple("gzip, deflate, br, chunked")]
fn transfer_encoding_is_chunked(header: TransferEncoding) -> bool {
    black_box(header).is_chunked()
}

#[library_benchmark(setup = typed::<ETag>)]
#[bench::strong("\"0123456789abcdef0123456789abcdef\"")]
#[bench::weak("W/\"0123456789abcdef0123456789abcdef\"")]
fn etag_is_weak(header: ETag) -> bool {
    black_box(header).is_weak()
}

fn matching_etags(raw: &str) -> (IfMatch, ETag) {
    let etag = typed::<ETag>(raw);
    (IfMatch::from(etag.clone()), etag)
}

fn matching_etag_list(list: &str, target: &str) -> (IfMatch, ETag) {
    (typed::<IfMatch>(list), typed::<ETag>(target))
}

#[library_benchmark(setup = matching_etags)]
#[bench::matching("\"0123456789abcdef0123456789abcdef\"")]
fn if_match_precondition_passes((condition, etag): (IfMatch, ETag)) -> bool {
    black_box(condition).precondition_passes(black_box(&etag))
}

#[library_benchmark]
#[bench::matching_last(
    args = (
        "\"first\", \"second\", \"0123456789abcdef0123456789abcdef\"",
        "\"0123456789abcdef0123456789abcdef\""
    ),
    setup = matching_etag_list
)]
fn if_match_list_precondition_passes((condition, etag): (IfMatch, ETag)) -> bool {
    black_box(condition).precondition_passes(black_box(&etag))
}

#[library_benchmark(setup = typed::<Range>)]
#[bench::ranges("bytes=0-1023,2048-4095,8192-,-4096")]
fn range_satisfiable_ranges(header: Range) -> Vec<(std::ops::Bound<u64>, std::ops::Bound<u64>)> {
    black_box(header)
        .satisfiable_ranges(black_box(16384))
        .collect()
}

#[library_benchmark(setup = typed::<ContentDisposition>)]
#[bench::attachment(
    "attachment; filename=\"a-very-long-file-name-used-to-exercise-prefix-parsing.tar.gz\""
)]
fn content_disposition_is_attachment(header: ContentDisposition) -> bool {
    black_box(header).is_attachment()
}

#[library_benchmark]
fn construct_range_bytes() -> Range {
    black_box(Range::bytes(black_box(0)..=black_box(1023)).unwrap())
}

#[library_benchmark(setup = typed::<Cookie>)]
#[bench::last("a=1; b=2; c=3; d=4; e=5; target=abcdefgh")]
fn cookie_get_last(header: Cookie) -> bool {
    black_box(header).get(black_box("target")) == Some("abcdefgh")
}

#[library_benchmark(setup = typed::<Cookie>)]
#[bench::six("a=1; b=2; c=3; d=4; e=5; target=abcdefgh")]
fn cookie_len(header: Cookie) -> usize {
    black_box(header).len()
}

library_benchmark_group!(
    name = decode;
    benchmarks =
        decode_content_length,
        decode_age,
        decode_cookie,
        decode_cache_control,
        decode_cache_control_many,
        decode_connection,
        decode_etag,
        decode_date,
        decode_sts,
        decode_host,
        decode_range,
        decode_content_range,
        decode_referrer_policy
);

library_benchmark_group!(
    name = encode;
    benchmarks =
        encode_content_length,
        encode_age,
        encode_cookie,
        encode_cache_control,
        encode_connection,
        encode_etag,
        encode_date,
        encode_sts,
        encode_host,
        encode_range,
        encode_content_range
);

library_benchmark_group!(
    name = query;
    benchmarks =
        transfer_encoding_is_chunked,
        etag_is_weak,
        if_match_precondition_passes,
        if_match_list_precondition_passes,
        range_satisfiable_ranges,
        content_disposition_is_attachment,
        construct_range_bytes,
        cookie_get_last,
        cookie_len
);

main!(library_benchmark_groups = decode, encode, query);
