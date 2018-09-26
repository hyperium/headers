use {HeaderValue};

/// `Referrer-Policy` header, part of
/// [Referrer Policy](https://www.w3.org/TR/referrer-policy/#referrer-policy-header)
///
/// The `Referrer-Policy` HTTP header specifies the referrer
/// policy that the user agent applies when determining what
/// referrer information should be included with requests made,
/// and with browsing contexts created from the context of the
/// protected resource.
///
/// # ABNF
///
/// ```text
/// Referrer-Policy: 1#policy-token
/// policy-token   = "no-referrer" / "no-referrer-when-downgrade"
///                  / "same-origin" / "origin"
///                  / "origin-when-cross-origin" / "unsafe-url"
/// ```
///
/// # Example values
///
/// * `no-referrer`
///
/// # Example
///
/// ```
/// # extern headers_ext as headers;
/// use headers::ReferrerPolicy;
///
/// let rp = ReferrerPolicy::NO_REFERRER;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Header)]
pub struct ReferrerPolicy(Policy);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Policy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    OriginWhenCrossOrigin,
    UnsafeUrl,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}

impl ReferrerPolicy {
    /// `no-referrer`
    pub const NO_REFERRER: Self = ReferrerPolicy(Policy::NoReferrer);

    /// `no-referrer-when-downgrade`
    pub const NO_REFERRER_WHEN_DOWNGRADE: Self = ReferrerPolicy(Policy::NoReferrerWhenDowngrade);

    /// `same-origin`
    pub const SAME_ORIGIN: Self = ReferrerPolicy(Policy::SameOrigin);

    /// `origin`
    pub const ORIGIN: Self = ReferrerPolicy(Policy::Origin);

    /// `origin-when-cross-origin`
    pub const ORIGIN_WHEN_CROSS_ORIGIN: Self = ReferrerPolicy(Policy::OriginWhenCrossOrigin);

    /// `unsafe-url`
    pub const UNSAFE_URL: Self = ReferrerPolicy(Policy::UnsafeUrl);

    /// `strict-origin`
    pub const STRICT_ORIGIN: Self = ReferrerPolicy(Policy::StrictOrigin);

    ///`strict-origin-when-cross-origin`
    pub const STRICT_ORIGIN_WHEN_CROSS_ORIGIN: Self = ReferrerPolicy(Policy::StrictOriginWhenCrossOrigin);
}

impl ::headers_core::decode::TryFromValues for Policy {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        // See https://www.w3.org/TR/referrer-policy/#determine-policy-for-token
        // tl;dr - Pick *last* known policy in the list
        values.skip_exhaustive_iter_check();
        for s in reverse_csv(values) {
            match s {
                "no-referrer" | "never" => return Some(Policy::NoReferrer),
                "no-referrer-when-downgrade" | "default" => return Some(Policy::NoReferrerWhenDowngrade),
                "same-origin" => return Some(Policy::SameOrigin),
                "origin" => return Some(Policy::Origin),
                "origin-when-cross-origin" => return Some(Policy::OriginWhenCrossOrigin),
                "strict-origin" => return Some(Policy::StrictOrigin),
                "strict-origin-when-cross-origin" => return Some(Policy::StrictOriginWhenCrossOrigin),
                "unsafe-url" | "always" => return Some(Policy::UnsafeUrl),
                _ => (),
            }
        }

        None
    }
}

impl<'a> From<&'a Policy> for HeaderValue {
    fn from(policy: &'a Policy) -> HeaderValue {
        HeaderValue::from_static(match *policy {
            Policy::NoReferrer => "no-referrer",
            Policy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            Policy::SameOrigin => "same-origin",
            Policy::Origin => "origin",
            Policy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            Policy::StrictOrigin => "strict-origin",
            Policy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            Policy::UnsafeUrl => "unsafe-url",
        })
    }
}

fn reverse_csv<'a, 'b>(values: &'a mut ::Values<'b>) -> impl Iterator<Item=&'b str> + 'a {
    values
        .rev()
        .flat_map(|value| {
            value
                .to_str()
                .into_iter()
                .flat_map(|string| {
                    string
                        .split(',')
                        .rev()
                        .filter_map(|x| match x.trim() {
                            "" => None,
                            y => Some(y),
                        })
                })
        })
}

#[cfg(test)]
mod tests {
    use super::ReferrerPolicy;
    use super::super::test_decode;

    #[test]
    fn decode_as_last_policy() {
        assert_eq!(
            test_decode::<ReferrerPolicy>(&["same-origin, origin"]),
            Some(ReferrerPolicy::ORIGIN),
        );

        assert_eq!(
            test_decode::<ReferrerPolicy>(&["origin", "same-origin"]),
            Some(ReferrerPolicy::SAME_ORIGIN),
        );
    }

    #[test]
    fn decode_as_last_known() {
        assert_eq!(
            test_decode::<ReferrerPolicy>(&["origin, nope, nope, nope"]),
            Some(ReferrerPolicy::ORIGIN),
        );

        assert_eq!(
            test_decode::<ReferrerPolicy>(&["nope, origin, nope, nope"]),
            Some(ReferrerPolicy::ORIGIN),
        );

        assert_eq!(
            test_decode::<ReferrerPolicy>(&["nope, origin", "nope, nope"]),
            Some(ReferrerPolicy::ORIGIN),
        );

        assert_eq!(
            test_decode::<ReferrerPolicy>(&["nope", "origin", "nope, nope"]),
            Some(ReferrerPolicy::ORIGIN),
        );
    }

    #[test]
    fn decode_unknown() {
        assert_eq!(
            test_decode::<ReferrerPolicy>(&["nope"]),
            None,
        );
    }

    #[test]
    fn matching() {
        let rp = ReferrerPolicy::ORIGIN;

        match rp {
            ReferrerPolicy::ORIGIN => (),
            _ => panic!("matched wrong"),
        }
    }
}
