use std::{
    fmt::{self, Write},
    iter,
    net::IpAddr,
    str::FromStr,
};

use headers_core::Error;
use http::{uri::Scheme, HeaderValue};

use crate::{
    util::{Comma, FlatCsv, SemiColon, TryFromValues},
    Header, Host,
};

/// A `for`/`by` parameter node name
#[derive(Clone, Debug, PartialEq)]
pub enum NodeName {
    /// The node IPv4 or IPv6 address.
    IpAddr(IpAddr),
    /// Signifies that the node name was unknown.
    Unknown,
    /// The node name was obfuscated by the proxy.
    ObfNode(String),
}

impl NodeName {
    fn needs_quoting(&self) -> bool {
        if let Self::IpAddr(addr) = self {
            addr.is_ipv6()
        } else {
            false
        }
    }
}

impl fmt::Display for NodeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IpAddr(addr) if addr.is_ipv6() => {
                f.write_char('[')?;
                f.write_str(&addr.to_string())?;
                f.write_char(']')
            }
            Self::IpAddr(addr) => f.write_str(&addr.to_string()),
            Self::Unknown => f.write_str("unknown"),
            Self::ObfNode(obfnode) => f.write_str(obfnode),
        }
    }
}

impl FromStr for NodeName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') && s.ends_with(']') {
            if let Ok(addr) = IpAddr::from_str(&s[1..s.len() - 1]) {
                return Ok(Self::IpAddr(addr));
            }
        } else if let Ok(addr) = IpAddr::from_str(s) {
            return Ok(Self::IpAddr(addr));
        }

        if s == "unknown" {
            return Ok(Self::Unknown);
        }

        if s.starts_with('_') {
            return Ok(Self::ObfNode(s.to_string()));
        }

        Err(Error::invalid())
    }
}

/// A `for`/`by` parameter node port
#[derive(Clone, Debug, PartialEq)]
pub enum NodePort {
    /// The node port number.
    Port(u16),
    /// The node port was obfuscatet by the proxy.
    ObfPort(String),
}

impl fmt::Display for NodePort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Port(port) => f.write_str(&port.to_string()),
            Self::ObfPort(obfport) => f.write_str(obfport),
        }
    }
}

impl FromStr for NodePort {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(port) = u16::from_str(s) {
            return Ok(Self::Port(port));
        }

        if s.starts_with('_') {
            return Ok(Self::ObfPort(s.to_string()));
        }

        Err(Error::invalid())
    }
}

/// A single node specification for use in a `Forwarded` header's `for` or `by` parameter
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// Name of the node.
    pub name: NodeName,
    /// Port of the node.
    pub port: Option<NodePort>,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If we're parsing an IPv6 address, only try to parse a port if the corresponding colon
        // occurrs after the end of the IPv6 address.
        let require_colon_after = s.find(']').unwrap_or(0);

        if let Some(colon) = s.rfind(':') {
            if colon > require_colon_after {
                let (name, port) = s.split_at(colon);
                return Ok(Self {
                    name: name.parse()?,
                    port: Some(port[1..].parse()?),
                });
            }
        }

        Ok(Self {
            name: s.parse()?,
            port: None,
        })
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // IPv6 addresses and nodes containing ports need to be quoted since `:` is not a valid
        // token character.
        let needs_quoting = self.name.needs_quoting() || self.port.is_some();
        if needs_quoting {
            f.write_char('"')?;
        }

        self.name.fmt(f)?;

        if let Some(port) = &self.port {
            f.write_char(':')?;
            port.fmt(f)?;
        }

        if needs_quoting {
            f.write_char('"')?;
        }

        Ok(())
    }
}

/// One element of a `Forwarded` header's list of parameter pairs
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForwardedElement {
    /// Identifies the node making the request to the proxy.
    pub r#for: Option<Node>,
    /// Identifies the user-agent facing interface of the proxy.
    pub by: Option<Node>,
    /// The protocol used to make the request.
    pub proto: Option<Scheme>,
    /// The host request header field as received by the proxy.
    pub host: Option<Host>,
}

/// `Forwarded` header, defined in [RFC7239](https://datatracker.ietf.org/doc/html/rfc7239)
///
/// # ABNF
///
/// ```text
/// Forwarded   = 1#forwarded-element
///
/// forwarded-element =
///     [ forwarded-pair ] *( ";" [ forwarded-pair ] )
///
/// forwarded-pair = token "=" value
/// value          = token / quoted-string
///
/// token = <Defined in [RFC7230], Section 3.2.6>
/// quoted-string = <Defined in [RFC7230], Section 3.2.6>
///
///
/// node     = nodename [ ":" node-port ]
/// nodename = IPv4address / "[" IPv6address "]" /
///             "unknown" / obfnode
///
/// IPv4address = <Defined in [RFC3986], Section 3.2.2>
/// IPv6address = <Defined in [RFC3986], Section 3.2.2>
/// obfnode = "_" 1*( ALPHA / DIGIT / "." / "_" / "-")
///
/// node-port     = port / obfport
/// port          = 1*5DIGIT
/// obfport       = "_" 1*(ALPHA / DIGIT / "." / "_" / "-")
///
/// DIGIT = <Defined in [RFC5234], Section 3.4>
/// ALPHA = <Defined in [RFC5234], Section B.1>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Forwarded(Vec<ForwardedElement>);

impl Forwarded {
    /// Iterate over the forwarded elements of this `Forwarded` header.
    pub fn iter(&self) -> impl Iterator<Item = &ForwardedElement> {
        self.0.iter()
    }
}

impl Header for Forwarded {
    fn name() -> &'static http::HeaderName {
        &::http::header::FORWARDED
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let mut parsed_elements = Vec::new();
        let elements = FlatCsv::<Comma>::try_from_values(values)?;
        for element in elements.iter() {
            let pairs = FlatCsv::<SemiColon>::from(
                HeaderValue::from_str(element).map_err(|_err| Error::invalid())?,
            );

            let mut element = ForwardedElement::default();
            for pair in pairs.iter() {
                let (parameter, value) = pair.split_once('=').ok_or_else(Error::invalid)?;
                if parameter.eq_ignore_ascii_case("for") {
                    let node: Node = value.trim_matches('"').parse()?;
                    if let Some(_old) = element.r#for.replace(node) {
                        return Err(Error::invalid());
                    }
                } else if parameter.eq_ignore_ascii_case("by") {
                    let node: Node = value.trim_matches('"').parse()?;
                    if let Some(_old) = element.by.replace(node) {
                        return Err(Error::invalid());
                    }
                } else if parameter.eq_ignore_ascii_case("proto") {
                    let scheme: Scheme = value
                        .trim_matches('"')
                        .parse()
                        .map_err(|_err| Error::invalid())?;
                    if let Some(_old) = element.proto.replace(scheme) {
                        return Err(Error::invalid());
                    }
                } else if parameter.eq_ignore_ascii_case("host") {
                    let value = HeaderValue::from_str(value).expect("Host is a valid HeaderValue");
                    let host = Host::decode(&mut iter::once(&value))?;
                    if let Some(_old) = element.host.replace(host) {
                        return Err(Error::invalid());
                    }
                } else {
                    return Err(Error::invalid());
                }
            }
            parsed_elements.push(element);
        }

        Ok(Self(parsed_elements))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        for element in &self.0 {
            let mut parts = Vec::new();
            if let Some(r#for) = &element.r#for {
                parts.push(format!("for={for}"));
            }
            if let Some(by) = &element.by {
                parts.push(format!("by={by}"));
            }
            if let Some(proto) = &element.proto {
                parts.push(format!("proto={proto}"));
            }
            if let Some(host) = &element.host {
                parts.push(format!("host={host}"));
            }
            let value = parts.join(";");
            let value = HeaderValue::from_str(&value).expect("Forwarded is a valid HeaderValue");

            values.extend(iter::once(value));
        }
    }
}

impl IntoIterator for Forwarded {
    type IntoIter = IntoIter;
    type Item = ForwardedElement;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter(std::vec::IntoIter<ForwardedElement>);

impl Iterator for IntoIter {
    type Item = ForwardedElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use http::header;
    use http::uri::Authority;

    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn test_parse() {
        let decoded = test_decode::<Forwarded>(&[
            r#"for=10.0.0.1;proto=https,by="10.0.2.1";for=10.0.1.1,host=localhost"#,
            "for=10.0.2.1;by=10.0.3.1:80",
        ])
        .unwrap();
        let expected = vec![
            ForwardedElement {
                r#for: Some("10.0.0.1".parse().unwrap()),
                proto: Some("https".parse().unwrap()),
                ..Default::default()
            },
            ForwardedElement {
                r#for: Some("10.0.1.1".parse().unwrap()),
                by: Some("10.0.2.1".parse().unwrap()),
                ..Default::default()
            },
            ForwardedElement {
                host: Some("localhost".parse::<Authority>().unwrap().into()),
                ..Default::default()
            },
            ForwardedElement {
                r#for: Some("10.0.2.1".parse().unwrap()),
                by: Some("10.0.3.1:80".parse().unwrap()),
                ..Default::default()
            },
        ];

        for (decoded, expected) in decoded.iter().zip(expected) {
            assert_eq!(*decoded, expected);
        }
    }

    #[test]
    fn rfc_examples() {
        let examples: [(&[&[_]], _, &[_]); 8] = [
            // 4.  Forwarded HTTP Header Field
            (
                &[&[r#"for="_gazonk""#]],
                vec![ForwardedElement {
                    r#for: Some(Node {
                        name: NodeName::ObfNode("_gazonk".to_string()),
                        port: None,
                    }),
                    ..Default::default()
                }],
                &[r#"for=_gazonk"#],
            ),
            (
                &[&[r#"For="[2001:db8:cafe::17]:4711""#]],
                vec![ForwardedElement {
                    r#for: Some(Node {
                        name: NodeName::IpAddr("2001:db8:cafe::17".parse().unwrap()),
                        port: Some(NodePort::Port(4711)),
                    }),
                    ..Default::default()
                }],
                &[r#"for="[2001:db8:cafe::17]:4711""#],
            ),
            (
                &[&[r#"for=192.0.2.60;proto=http;by=203.0.113.43"#]],
                vec![ForwardedElement {
                    r#for: Some(Node {
                        name: NodeName::IpAddr("192.0.2.60".parse().unwrap()),
                        port: None,
                    }),
                    by: Some(Node {
                        name: NodeName::IpAddr("203.0.113.43".parse().unwrap()),
                        port: None,
                    }),
                    proto: Some(Scheme::HTTP),
                    ..Default::default()
                }],
                &[r#"for=192.0.2.60;by=203.0.113.43;proto=http"#],
            ),
            (
                &[&[r#"for=192.0.2.43, for=198.51.100.17"#]],
                vec![
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("192.0.2.43".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("198.51.100.17".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                ],
                &["for=192.0.2.43", "for=198.51.100.17"],
            ),
            // 6.3.  Obfuscated Identifier
            (
                &[&["for=_hidden, for=_SEVKISEK"]],
                vec![
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::ObfNode("_hidden".to_string()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::ObfNode("_SEVKISEK".to_string()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                ],
                &["for=_hidden", "for=_SEVKISEK"],
            ),
            // 7.1.  HTTP Lists
            (
                &[
                    &[r#"for=192.0.2.43,for="[2001:db8:cafe::17]",for=unknown"#],
                    &[r#"for=192.0.2.43, for="[2001:db8:cafe::17]", for=unknown"#],
                    &[
                        "for=192.0.2.43",
                        r#"for="[2001:db8:cafe::17]", for=unknown"#,
                    ],
                ],
                vec![
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("192.0.2.43".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("2001:db8:cafe::17".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::Unknown,
                            port: None,
                        }),
                        ..Default::default()
                    },
                ],
                &[
                    "for=192.0.2.43",
                    r#"for="[2001:db8:cafe::17]""#,
                    "for=unknown",
                ],
            ),
            // 7.4.  Transition
            (
                &[&[r#"for=192.0.2.43, for="[2001:db8:cafe::17]""#]],
                vec![
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("192.0.2.43".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("2001:db8:cafe::17".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                ],
                &["for=192.0.2.43", r#"for="[2001:db8:cafe::17]""#],
            ),
            // 7.5.  Example Usage
            (
                &[&[
                    "for=192.0.2.43,for=198.51.100.17;by=203.0.113.60;proto=http;host=example.com",
                ]],
                vec![
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("192.0.2.43".parse().unwrap()),
                            port: None,
                        }),
                        ..Default::default()
                    },
                    ForwardedElement {
                        r#for: Some(Node {
                            name: NodeName::IpAddr("198.51.100.17".parse().unwrap()),
                            port: None,
                        }),
                        by: Some(Node {
                            name: NodeName::IpAddr("203.0.113.60".parse().unwrap()),
                            port: None,
                        }),
                        proto: Some(Scheme::HTTP),
                        host: Some("example.com".parse::<Authority>().unwrap().into()),
                        ..Default::default()
                    },
                ],
                &[
                    "for=192.0.2.43",
                    "for=198.51.100.17;by=203.0.113.60;proto=http;host=example.com",
                ],
            ),
        ];

        for (idx, (headers, expected, roundtripped)) in
            IntoIterator::into_iter(examples).enumerate()
        {
            let expected = Forwarded(expected);
            for headers in headers {
                eprintln!("{idx} {headers:?}");
                assert_eq!(
                    test_decode::<Forwarded>(headers).unwrap(),
                    expected,
                    "Decode test failed for example {}/{headers:?}",
                    idx + 1,
                );
            }

            let roundtripped = roundtripped
                .into_iter()
                .map(|v| (header::FORWARDED, HeaderValue::from_static(v)))
                .collect();
            assert_eq!(
                test_encode(expected),
                roundtripped,
                "Roundtrip test failed for example {}",
                idx + 1,
            );
        }
    }
}
