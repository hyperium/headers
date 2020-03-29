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
        }

        impl std::string::ToString for ContentCoding {
            #[inline]
            fn to_string(&self) -> String {
                match *self {
                    $(ContentCoding::$coding => $str.to_string(),)+
                }
            }
        }

        impl std::str::FromStr for ContentCoding {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($coding)
                        | $str => Ok(ContentCoding::$coding),
                    )+
                    _ => Err("invalid content coding")
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
    use std::str::FromStr;

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
        assert_eq!(ContentCoding::from_str("br"), Ok(ContentCoding::BROTLI));
        assert_eq!(ContentCoding::from_str("GZIP"), Ok(ContentCoding::GZIP));
        assert_eq!(ContentCoding::from_str("blah blah"), Err("invalid content coding"));
    }
}