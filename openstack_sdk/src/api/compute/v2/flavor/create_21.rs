//! Creates a flavor.
//!
//! Creating a flavor is typically only available to administrators of a
//! cloud because this has implications for scheduling efficiently in the
//! cloud.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

/// The ID and links for the flavor for your server instance. A flavor is a
/// combination
/// of memory, disk size, and CPUs.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Flavor<'a> {
    /// The display name of a flavor.
    #[serde()]
    #[builder(setter(into))]
    name: Cow<'a, str>,

    /// Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces
    /// and dots ‘.’ are permitted. If an ID is not provided, then a default
    /// UUID
    /// will be assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    id: Option<Option<Cow<'a, str>>>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[builder(setter(into))]
    ram: Cow<'a, str>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[builder(setter(into))]
    vcpus: Cow<'a, str>,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[serde()]
    #[builder(setter(into))]
    disk: Cow<'a, str>,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[serde(
        rename = "OS-FLV-EXT-DATA:ephemeral",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    os_flv_ext_data_ephemeral: Option<Cow<'a, str>>,

    /// The size of a dedicated swap disk that will be allocated, in
    /// MiB. If 0 (the default), no dedicated swap disk will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    swap: Option<Cow<'a, str>>,

    /// The receive / transmit factor (as a float) that will be set on
    /// ports if the network backend supports the QOS extension.
    /// Otherwise it will be ignored. It defaults to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    rxtx_factor: Option<Cow<'a, str>>,

    /// Whether the flavor is public (available to all projects) or scoped
    /// to a set of projects. Default is True if not specified.
    #[serde(
        rename = "os-flavor-access:is_public",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    os_flavor_access_is_public: Option<bool>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The ID and links for the flavor for your server instance. A flavor is a
    /// combination
    /// of memory, disk size, and CPUs.
    #[builder(setter(into))]
    flavor: Flavor<'a>,

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
    /// Add a single header to the Flavor.
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
        "v2.1/flavors".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("flavor", serde_json::to_value(&self.flavor)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavor".into())
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
                .flavor(
                    FlavorBuilder::default()
                        .name("foo")
                        .ram("foo")
                        .vcpus("foo")
                        .disk("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .flavor(
                    FlavorBuilder::default()
                        .name("foo")
                        .ram("foo")
                        .vcpus("foo")
                        .disk("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "flavor"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.1/flavors",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavor": {} }));
        });

        let endpoint = Request::builder()
            .flavor(
                FlavorBuilder::default()
                    .name("foo")
                    .ram("foo")
                    .vcpus("foo")
                    .disk("foo")
                    .build()
                    .unwrap(),
            )
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
                .path(format!("/v2.1/flavors",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavor": {} }));
        });

        let endpoint = Request::builder()
            .flavor(
                FlavorBuilder::default()
                    .name("foo")
                    .ram("foo")
                    .vcpus("foo")
                    .disk("foo")
                    .build()
                    .unwrap(),
            )
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