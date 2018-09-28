/// The `Sec-Websocket-Version` header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SecWebsocketVersion(u8);

impl SecWebsocketVersion {
    /// `Sec-Websocket-Version: 13`
    pub const V13: SecWebsocketVersion = SecWebsocketVersion(13);
}

impl ::Header for SecWebsocketVersion {
    const NAME: &'static ::HeaderName = &::http::header::SEC_WEBSOCKET_VERSION;

    fn decode(values: &mut ::Values) -> Option<Self> {
        let value = values.next()?;

        if value == "13" {
            Some(SecWebsocketVersion::V13)
        } else {
            None
        }
    }

    fn encode(&self, values: &mut ::ToValues) {
        debug_assert_eq!(self.0, 13);

        values.append(::HeaderValue::from_static("13"));
    }
}

#[cfg(test)]
mod tests {
    use super::SecWebsocketVersion;
    use super::super::{test_decode, test_encode};

    #[test]
    fn decode_v13() {
        assert_eq!(
            test_decode::<SecWebsocketVersion>(&["13"]),
            Some(SecWebsocketVersion::V13),
        );
    }

    #[test]
    fn decode_fail() {
        assert_eq!(
            test_decode::<SecWebsocketVersion>(&["1"]),
            None,
        );
    }

    #[test]
    fn encode_v13() {
        let headers = test_encode(SecWebsocketVersion::V13);
        assert_eq!(headers["sec-websocket-version"], "13");
    }
}
