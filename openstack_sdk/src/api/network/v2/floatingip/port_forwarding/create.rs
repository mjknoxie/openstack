//! Creates a floating IP port forwarding.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    #[serde(rename = "dccp")]
    Dccp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "ipv6-icmp")]
    Ipv6Icmp,
    #[serde(rename = "sctp")]
    Sctp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

/// A `floating IP port forwarding` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct PortForwarding<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP
    /// address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    external_port: Option<Option<f32>>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    internal_port: Option<Option<f32>>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP
    /// port forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    internal_ip_address: Option<Cow<'a, str>>,

    /// The IP protocol used in the floating IP port forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    protocol: Option<Protocol>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    internal_port_id: Option<Cow<'a, str>>,

    /// A text describing the rule, which helps users to
    /// manage/find easily theirs rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP
    /// address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    internal_port_range: Option<f32>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `floating IP port forwarding` object.
    #[builder(setter(into))]
    port_forwarding: PortForwarding<'a>,

    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    #[builder(default, setter(into))]
    floatingip_id: Cow<'a, str>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Port_Forwarding.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.0/floatingips/{floatingip_id}/port_forwardings",
            floatingip_id = self.floatingip_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "port_forwarding",
            serde_json::to_value(&self.port_forwarding)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("port_forwarding".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .port_forwarding(PortForwardingBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .port_forwarding(PortForwardingBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "port_forwarding"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.0/floatingips/{floatingip_id}/port_forwardings",
                floatingip_id = "floatingip_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port_forwarding": {} }));
        });

        let endpoint = Request::builder()
            .floatingip_id("floatingip_id")
            .port_forwarding(PortForwardingBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!(
                    "/v2.0/floatingips/{floatingip_id}/port_forwardings",
                    floatingip_id = "floatingip_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port_forwarding": {} }));
        });

        let endpoint = Request::builder()
            .floatingip_id("floatingip_id")
            .port_forwarding(PortForwardingBuilder::default().build().unwrap())
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .iter()
                .cloned(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}