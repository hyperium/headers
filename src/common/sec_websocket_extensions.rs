use std::{borrow::Cow, fmt::Debug, iter::FromIterator, str::FromStr};

use bytes::BytesMut;
use headers_core::Error;
use http::HeaderValue;

use crate::util::{csv, TryFromValues};

/// The `Sec-Websocket-Extensions` header.
///
/// This header is used in the Websocket handshake, sent by the client to the
/// server and then from the server to the client. It is a proposed and
/// agreed-upon list of websocket protocol extensions to use.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SecWebsocketExtensions(Vec<WebsocketProtocolExtension>);

/// An extension listed in a [`SecWebsocketExtensions`] header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WebsocketProtocolExtension {
    name: Cow<'static, str>,
    params: Vec<WebsocketExtensionParam>,
}

/// Named parameter for an extension in a `Sec-Websocket-Extensions` header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WebsocketExtensionParam {
    name: Cow<'static, str>,
    value: Option<String>,
}

impl SecWebsocketExtensions {
    /// Constructs a new header with the provided extensions.
    pub fn new(extensions: impl IntoIterator<Item = WebsocketProtocolExtension>) -> Self {
        Self(extensions.into_iter().collect())
    }

    /// Returns an iterator over the extensions in this header.
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl WebsocketProtocolExtension {
    /// Constructs a new extension directive with the given name and parameters.
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        params: impl IntoIterator<Item = WebsocketExtensionParam>,
    ) -> Self {
        Self {
            name: name.into(),
            params: params.into_iter().collect(),
        }
    }

    /// The name of this extension directive.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns an iterator over the parameters for this extension directive.
    pub fn params(&self) -> impl Iterator<Item = &WebsocketExtensionParam> {
        self.params.iter()
    }
}

impl WebsocketExtensionParam {
    /// Constructs a new parameter with the given name and optional value.
    #[inline]
    pub fn new(name: impl Into<Cow<'static, str>>, value: Option<String>) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    /// The name of the parameter.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The parameter value, if there is one.
    #[inline]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl crate::Header for SecWebsocketExtensions {
    fn name() -> &'static ::http::header::HeaderName {
        &::http::header::SEC_WEBSOCKET_EXTENSIONS
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, crate::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        crate::util::TryFromValues::try_from_values(values).map(SecWebsocketExtensions)
    }
    fn encode<E: Extend<crate::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(to_header_value(&self.0)));
    }
}

impl TryFromValues for Vec<WebsocketProtocolExtension> {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        csv::from_comma_delimited(values)
    }
}

impl FromIterator<WebsocketProtocolExtension> for SecWebsocketExtensions {
    fn from_iter<T: IntoIterator<Item = WebsocketProtocolExtension>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for SecWebsocketExtensions {
    type Item = WebsocketProtocolExtension;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SecWebsocketExtensions {
    type Item = &'a WebsocketProtocolExtension;

    type IntoIter = std::slice::Iter<'a, WebsocketProtocolExtension>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromStr for WebsocketProtocolExtension {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, tail) = s
            .split_once(';')
            .map(|(n, t)| (n, Some(t)))
            .unwrap_or((s, None));

        let params = csv::from_delimited(&mut tail.into_iter(), ';')?;

        Ok(Self {
            name: name.trim().to_owned().into(),
            params,
        })
    }
}

impl std::fmt::Display for WebsocketProtocolExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, params } = self;

        write!(f, "{}", name)?;
        for param in params {
            f.write_str("; ")?;
            write!(f, "{}", param)?;
        }

        Ok(())
    }
}

impl FromStr for WebsocketExtensionParam {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s
            .split_once('=')
            .map(|(n, t)| (n, Some(t)))
            .unwrap_or((s, None));

        let value = value.map(|value| value.trim().to_owned());

        Ok(Self {
            name: name.trim().to_owned().into(),
            value,
        })
    }
}

impl std::fmt::Display for WebsocketExtensionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, value } = self;

        write!(f, "{}", name)?;
        if let Some(value) = value {
            write!(f, "={}", value)?;
        }
        Ok(())
    }
}

impl WebsocketProtocolExtension {
    fn encoded_len(&self) -> usize {
        let Self { name, params } = self;

        let params_len: usize = params.iter().map(|p| p.encoded_len() + 2).sum();

        name.len() + params_len
    }
}

impl WebsocketExtensionParam {
    fn encoded_len(&self) -> usize {
        let Self { name, value } = self;
        name.len() + value.as_ref().map(|s| s.len() + 1).unwrap_or_default()
    }

    fn write_to_buffer(&self, buffer: &mut BytesMut) {
        let Self { name, value } = self;
        buffer.extend_from_slice(b"; ");
        buffer.extend_from_slice(name.as_bytes());

        if let Some(value) = value {
            buffer.extend_from_slice(b"=");
            buffer.extend_from_slice(value.as_bytes());
        }
    }
}

fn to_header_value(extensions: &[WebsocketProtocolExtension]) -> HeaderValue {
    let mut buffer = BytesMut::with_capacity(encoded_len(extensions));

    for extension in extensions {
        if !buffer.is_empty() {
            buffer.extend_from_slice(b", ");
        }

        let WebsocketProtocolExtension { name, params } = extension;
        buffer.extend_from_slice(name.as_bytes());

        for param in params {
            param.write_to_buffer(&mut buffer);
        }
    }

    HeaderValue::from_maybe_shared(buffer).expect("valid construction")
}

fn encoded_len(extensions: &[WebsocketProtocolExtension]) -> usize {
    let all_encoded_len: usize = extensions
        .iter()
        .map(WebsocketProtocolExtension::encoded_len)
        .sum();
    let all_separators_len = extensions
        .len()
        .checked_sub(1)
        .map(|num_separators| num_separators * 2)
        .unwrap_or_default();
    all_encoded_len + all_separators_len
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::Header;

    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn parse_separate_headers() {
        // From https://tools.ietf.org/html/rfc6455#section-9.1
        let extensions =
            test_decode::<SecWebsocketExtensions>(&["foo", "bar; baz=2"]).expect("valid");

        assert_eq!(
            extensions,
            SecWebsocketExtensions(vec![
                WebsocketProtocolExtension {
                    name: "foo".into(),
                    params: vec![],
                },
                WebsocketProtocolExtension {
                    name: "bar".into(),
                    params: vec![WebsocketExtensionParam {
                        name: "baz".into(),
                        value: Some("2".to_owned())
                    }],
                }
            ])
        );
    }

    #[test]
    fn round_trip_complex() {
        let extensions = test_decode::<SecWebsocketExtensions>(&[
            "deflate-stream",
            "mux; max-channels=4; flow-control, deflate-stream",
            "private-extension",
        ])
        .expect("valid");

        let headers = test_encode(extensions);
        assert_eq!(
            headers["sec-websocket-extensions"],
            "deflate-stream, mux; max-channels=4; flow-control, deflate-stream, private-extension"
        );
    }

    #[test]
    fn to_header_value_exact() {
        // This isn't a required property for correctness but if the length
        // precomputation is wrong we'll over- or under-allocate during
        // conversion.
        let cases = [
            SecWebsocketExtensions::new([
                WebsocketProtocolExtension::from_str("extension-name").unwrap(),
                WebsocketProtocolExtension::from_str("with-params; a=5; b=8").unwrap(),
            ]),
            SecWebsocketExtensions::new([]),
            SecWebsocketExtensions::new([
                WebsocketProtocolExtension::from_str("duplicate-name").unwrap(),
                WebsocketProtocolExtension::from_str("duplicate-name").unwrap(),
                WebsocketProtocolExtension::from_str("duplicate-name").unwrap(),
            ]),
        ];

        for case in cases {
            let mut values = Vec::new();
            case.encode(&mut values);
            let [value] = values.try_into().unwrap();

            assert_eq!(value.len(), encoded_len(&case.0));
        }
    }
}
