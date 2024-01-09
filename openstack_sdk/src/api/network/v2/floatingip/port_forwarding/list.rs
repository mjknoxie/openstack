//! Lists floating IP port forwardings that the project has access to.
//!
//! Default policy settings return only the port forwardings associated to
//! floating
//! IPs owned by the project of the user submitting the request, unless the
//! user has administrative role.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body.
//! Additionally, you can filter results by using query string parameters.
//! For information, see [Filtering and Column Selection](https://wiki.openstac
//! k.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    #[builder(setter(into), default)]
    floatingip_id: Cow<'a, str>,

    /// id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(setter(into), default)]
    id: Option<Cow<'a, str>>,

    /// external_port query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(default)]
    external_port: Option<f32>,

    /// protocol query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(setter(into), default)]
    protocol: Option<Cow<'a, str>>,

    /// internal_port_id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(setter(into), default)]
    internal_port_id: Option<Cow<'a, str>>,

    /// description query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,

    /// external_port_range query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    #[builder(default)]
    external_port_range: Option<f32>,

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
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.0/floatingips/{floatingip_id}/port_forwardings",
            floatingip_id = self.floatingip_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("external_port", self.external_port);
        params.push_opt("protocol", self.protocol.as_ref());
        params.push_opt("internal_port_id", self.internal_port_id.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("external_port_range", self.external_port_range);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("port_forwardings".into())
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "port_forwardings"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!(
                "/v2.0/floatingips/{floatingip_id}/port_forwardings",
                floatingip_id = "floatingip_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port_forwardings": {} }));
        });

        let endpoint = Request::builder()
            .floatingip_id("floatingip_id")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!(
                    "/v2.0/floatingips/{floatingip_id}/port_forwardings",
                    floatingip_id = "floatingip_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port_forwardings": {} }));
        });

        let endpoint = Request::builder()
            .floatingip_id("floatingip_id")
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
