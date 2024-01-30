//! Lists users.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/users`
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
    /// Filters the response by a domain ID.
    #[builder(default, setter(into))]
    domain_id: Option<Cow<'a, str>>,

    /// If set to true, then only enabled projects will be returned. Any value
    /// other than 0 (including no value) will be interpreted as true.
    #[builder(default)]
    enabled: Option<bool>,

    /// Filters the response by a domain ID.
    #[builder(default, setter(into))]
    idp_id: Option<Cow<'a, str>>,

    /// Filters the response by a resource name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// Filter results based on which user passwords have expired. The query
    /// should include an operator and a timestamp with a colon (:) separating
    /// the two, for example: `password_expires_at={operator}:{timestamp}`.
    /// Valid operators are: `lt`, `lte`, `gt`, `gte`, `eq`, and `neq`.
    /// Valid timestamps are of the form: YYYY-MM-DDTHH:mm:ssZ.
    #[builder(default, setter(into))]
    password_expires_at: Option<Cow<'a, str>>,

    /// Filters the response by a protocol ID.
    #[builder(default, setter(into))]
    protocol_id: Option<Cow<'a, str>>,

    /// Filters the response by a unique ID.
    #[builder(default, setter(into))]
    unique_id: Option<Cow<'a, str>>,

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
    /// Add a single header to the User.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v3/users".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("domain_id", self.domain_id.as_ref());
        params.push_opt("enabled", self.enabled);
        params.push_opt("idp_id", self.idp_id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("password_expires_at", self.password_expires_at.as_ref());
        params.push_opt("protocol_id", self.protocol_id.as_ref());
        params.push_opt("unique_id", self.unique_id.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("users".into())
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
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "users"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/users".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "users": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/users".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "users": {} }));
        });

        let endpoint = Request::builder()
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