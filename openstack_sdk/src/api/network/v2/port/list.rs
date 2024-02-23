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

//! Lists ports to which the user has access.
//!
//! Default policy settings return only those ports that are owned by the
//! project of the user who submits the request, unless the request is
//! submitted by a user with administrative rights.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body. Additionally, you can filter results by using query
//! string parameters. For information, see
//! [Filtering and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
//!
//! If the `ip-substring-filtering` extension is enabled, the Neutron API
//! supports IP address substring filtering on the `fixed_ips` attribute. If
//! you specify an IP address substring (`ip_address_substr`) in an entry of
//! the `fixed_ips` attribute, the Neutron API will list all ports that have an
//! IP address matching the substring.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use crate::api::common::CommaSeparatedList;
use std::borrow::Cow;
use std::collections::BTreeSet;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// id query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// name query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// network_id query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// admin_state_up query parameter for /v2.0/ports API
    ///
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// mac_address query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    mac_address: Option<Cow<'a, str>>,

    /// fixed_ips query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_fixed_ips"))]
    fixed_ips: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// device_id query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    device_id: Option<Cow<'a, str>>,

    /// device_owner query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    device_owner: Option<Cow<'a, str>>,

    /// tenant_id query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// status query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// ip_allocation query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    ip_allocation: Option<Cow<'a, str>>,

    /// binding:host_id query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    binding_host_id: Option<Cow<'a, str>>,

    /// revision_number query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    revision_number: Option<Cow<'a, str>>,

    /// tags query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tags-any query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_tags_any"))]
    tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags-any query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_not_tags_any"))]
    not_tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// description query parameter for /v2.0/ports API
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// security_groups query parameter for /v2.0/ports API
    ///
    #[builder(default, private, setter(name = "_security_groups"))]
    security_groups: BTreeSet<Cow<'a, str>>,

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
    /// fixed_ips query parameter for /v2.0/ports API
    ///
    pub fn fixed_ips<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.fixed_ips
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// tags query parameter for /v2.0/ports API
    ///
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// tags-any query parameter for /v2.0/ports API
    ///
    pub fn tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags query parameter for /v2.0/ports API
    ///
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags-any query parameter for /v2.0/ports API
    ///
    pub fn not_tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// security_groups query parameter for /v2.0/ports API
    ///
    pub fn security_groups<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.security_groups
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Port.
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
        "v2.0/ports".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("network_id", self.network_id.as_ref());
        params.push_opt("admin_state_up", self.admin_state_up);
        params.push_opt("mac_address", self.mac_address.as_ref());
        params.push_opt("fixed_ips", self.fixed_ips.as_ref());
        params.push_opt("device_id", self.device_id.as_ref());
        params.push_opt("device_owner", self.device_owner.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("ip_allocation", self.ip_allocation.as_ref());
        params.push_opt("binding:host_id", self.binding_host_id.as_ref());
        params.push_opt("revision_number", self.revision_number.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.extend(
            self.security_groups
                .iter()
                .map(|value| ("security_groups", value)),
        );

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("ports".into())
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "ports"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/ports".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ports": {} }));
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
                .path("/v2.0/ports".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ports": {} }));
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
