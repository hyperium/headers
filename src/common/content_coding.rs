macro_rules! define_content_coding {
    ($($coding:ident; $str:expr,)+) => {   
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        /// Values that are used with headers like `Content-Encoding`
        /// [RFC7231](https://www.iana.org/assignments/http-parameters/http-parameters.xhtml)
        /// 
        pub enum ContentCoding {
            $(
                #[doc = $str]
                $coding,
            )+
        }

        impl ContentCoding {
            /// Returns a static str for a ContentCoding
            #[inline]
            pub fn to_static(&self) -> &'static str {
                match *self {
                    $(ContentCoding::$coding => $str,)+
                }
            }

            /// Given a &str returns a ContentCoding. 
            /// 
            /// Note this will never fail, in the case of `&str` being an invalid content coding, 
            /// will return `ContentCoding::IDENTITY` because identity is generally always an 
            /// accepted coding.
            /// 
            /// # Example
            /// 
            /// ```
            /// use headers::ContentCoding;
            /// 
            /// let invalid = ContentCoding::from_str("not a valid coding");
            /// assert_eq!(invalid, ContentCoding::IDENTITY);
            /// 
            /// let valid = ContentCoding::from_str("gzip");
            /// assert_eq!(valid, ContentCoding::GZIP);
            /// ```
            /// 
            #[inline]
            pub fn from_str(s: &str) -> Self {
                ContentCoding::try_from_str(s).unwrap_or_else(|_| ContentCoding::IDENTITY)
            }

            #[inline]
            /// Given a &str will try to return a ContentCoding
            /// 
            /// Different from `ContentCoding::from_str(&str)`, if `&str` is an invalid content
            /// coding, it will return `Err(())`
            /// 
            /// # Example
            /// 
            /// ```
            /// use headers::ContentCoding;
            /// 
            /// let invalid = ContentCoding::try_from_str("not a valid coding");
            /// assert!(invalid.is_err());
            /// 
            /// let valid = ContentCoding::try_from_str("gzip");
            /// assert_eq!(valid.unwrap(), ContentCoding::GZIP);
            /// ```
            /// 
            pub fn try_from_str(s: &str) -> Result<Self, ()> {
                match s {
                    $(
                        stringify!($coding)
                        | $str => Ok(ContentCoding::$coding),
                    )+
                    _ => Err(())
                }
            }
        }

        impl std::string::ToString for ContentCoding {
            #[inline]
            fn to_string(&self) -> String {
                match *self {
                    $(ContentCoding::$coding => $str.to_string(),)+
                }
            }
        }
    }
}

define_content_coding! {
    BROTLI; "br",
    COMPRESS; "compress",
    DEFLATE; "deflate",
    GZIP; "gzip",
    IDENTITY; "identity",
}

#[cfg(test)]
mod tests {
    use super::ContentCoding;

    #[test]
    fn to_static() {
        assert_eq!(ContentCoding::GZIP.to_static(), "gzip");
    }

    #[test]
    fn to_string() {
        assert_eq!(ContentCoding::DEFLATE.to_string(), "deflate".to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(ContentCoding::from_str("br"), ContentCoding::BROTLI);
        assert_eq!(ContentCoding::from_str("GZIP"), ContentCoding::GZIP);
        assert_eq!(ContentCoding::from_str("blah blah"), ContentCoding::IDENTITY);
    }

    #[test]
    fn try_from_str() {
        assert_eq!(ContentCoding::try_from_str("br"), Ok(ContentCoding::BROTLI));
        assert_eq!(ContentCoding::try_from_str("blah blah"), Err(()));
    }
}