// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Creates a resource type association between a namespace and the resource
//! type specified in the body of the request.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 404, 409
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Resource type names should be aligned with Heat resource types whenever
    /// possible:
    /// https://docs.openstack.org/heat/latest/template_guide/openstack.html
    ///
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// Prefix for any properties in the namespace that you want to apply to
    /// the resource type. If you specify a prefix, you must append a prefix
    /// separator, such as the colon (`:`) character.
    ///
    #[builder(default, setter(into))]
    pub(crate) prefix: Option<Cow<'a, str>>,

    /// Some resource types allow more than one key and value pair for each
    /// instance. For example, the Image service allows both user and image
    /// metadata on volumes. The `properties_target` parameter enables a
    /// namespace target to remove the ambiguity.
    ///
    #[builder(default, setter(into))]
    pub(crate) properties_target: Option<Cow<'a, str>>,

    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type}
    /// API
    ///
    #[builder(default, setter(into))]
    namespace_name: Cow<'a, str>,

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
    /// Add a single header to the Resource_Type.
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
            "v2/metadefs/namespaces/{namespace_name}/resource_types",
            namespace_name = self.namespace_name.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("name", serde_json::to_value(&self.name)?);
        if let Some(val) = &self.prefix {
            params.push("prefix", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.properties_target {
            params.push("properties_target", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("resource_type_associations".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .name("foo")
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .name("foo")
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "resource_type_associations"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2/metadefs/namespaces/{namespace_name}/resource_types",
                namespace_name = "namespace_name",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource_type_associations": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .name("foo")
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
                    "/v2/metadefs/namespaces/{namespace_name}/resource_types",
                    namespace_name = "namespace_name",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource_type_associations": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .name("foo")
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}