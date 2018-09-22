use util::FlatCsv;

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
#[derive(Clone, Debug, PartialEq, Header)]
pub struct ReferrerPolicy(FlatCsv);

impl ReferrerPolicy {
    pub fn iter(&self) -> impl Iterator<Item=&str> {
        self.0.iter()
    }
}

/*
    /// `no-referrer`
    NoReferrer,
    /// `no-referrer-when-downgrade`
    NoReferrerWhenDowngrade,
    /// `same-origin`
    SameOrigin,
    /// `origin`
    Origin,
    /// `origin-when-cross-origin`
    OriginWhenCrossOrigin,
    /// `unsafe-url`
    UnsafeUrl,
     /// `strict-origin`
    StrictOrigin,
    ///`strict-origin-when-cross-origin`
    StrictOriginWhenCrossOrigin,
*/

