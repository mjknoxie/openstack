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

//! Creates a L7 rule.
//!
//! This operation provisions a new L7 rule by using the configuration that you
//! define in the request object. After the API validates the request and
//! starts the provisioning process, the API returns a response object that
//! contains a unique ID and the status of provisioning the L7 rule.
//!
//! In the response, the L7 rule [provisioning status](#prov-status) is
//! `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
//!
//! If the status is `PENDING_CREATE`, issue GET
//! `/v2/lbaas/l7policies/{l7policy_id}/rules/{l7rule_id}` to view the progress
//! of the provisioning operation. When the L7 rule status changes to `ACTIVE`,
//! the L7 rule is successfully provisioned and is ready for further
//! configuration.
//!
//! If the API cannot fulfill the request due to insufficient data or data that
//! is not valid, the service returns the HTTP `Bad Request (400)` response
//! code with information about the failure in the response body. Validation
//! errors require that you correct the error and submit the request again.
//!
//! All the rules associated with a given policy are logically ANDead together.
//! A request must match all the policy’s rules to match the policy.
//!
//! If you need to express a logical OR operation between rules, then do this
//! by creating multiple policies with the same action.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "COOKIE")]
    Cookie,
    #[serde(rename = "FILE_TYPE")]
    FileType,
    #[serde(rename = "HEADER")]
    Header,
    #[serde(rename = "HOST_NAME")]
    HostName,
    #[serde(rename = "PATH")]
    Path,
    #[serde(rename = "SSL_CONN_HAS_CERT")]
    SslConnHasCert,
    #[serde(rename = "SSL_DN_FIELD")]
    SslDnField,
    #[serde(rename = "SSL_VERIFY_RESULT")]
    SslVerifyResult,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum CompareType {
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    #[serde(rename = "EQUAL_TO")]
    EqualTo,
    #[serde(rename = "REGEX")]
    Regex,
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
}

/// Defines mandatory and optional attributes of a POST request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Rule<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`,
    /// `EQUAL_TO`, `REGEX`, or `STARTS_WITH`.
    ///
    #[serde()]
    #[builder()]
    pub(crate) compare_type: CompareType,

    /// When `true` the logic of the rule is inverted. For example, with invert
    /// `true`, equal to would become not equal to. Default is `false`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) invert: Option<bool>,

    /// The key to use for the comparison. For example, the name of the cookie
    /// to evaluate.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) key: Option<Cow<'a, str>>,

    /// The ID of the project owning this resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project_id: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

    /// The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`,
    /// `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`.
    ///
    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,

    /// The value to use for the comparison. For example, the file type to
    /// compare.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) value: Cow<'a, str>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[builder(setter(into))]
    pub(crate) rule: Rule<'a>,

    /// l7policy_id parameter for
    /// /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
    ///
    #[builder(default, setter(into))]
    l7policy_id: Cow<'a, str>,

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
    /// Add a single header to the Rule.
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
            "v2/lbaas/l7policies/{l7policy_id}/rules",
            l7policy_id = self.l7policy_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("rule", serde_json::to_value(&self.rule)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("rule".into())
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
                .rule(
                    RuleBuilder::default()
                        ._type(Type::Cookie)
                        .compare_type(CompareType::Contains)
                        .value("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::LoadBalancer
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .rule(
                    RuleBuilder::default()
                        ._type(Type::Cookie)
                        .compare_type(CompareType::Contains)
                        .value("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "rule"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2/lbaas/l7policies/{l7policy_id}/rules",
                l7policy_id = "l7policy_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "rule": {} }));
        });

        let endpoint = Request::builder()
            .l7policy_id("l7policy_id")
            .rule(
                RuleBuilder::default()
                    ._type(Type::Cookie)
                    .compare_type(CompareType::Contains)
                    .value("foo")
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
                .path(format!(
                    "/v2/lbaas/l7policies/{l7policy_id}/rules",
                    l7policy_id = "l7policy_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "rule": {} }));
        });

        let endpoint = Request::builder()
            .l7policy_id("l7policy_id")
            .rule(
                RuleBuilder::default()
                    ._type(Type::Cookie)
                    .compare_type(CompareType::Contains)
                    .value("foo")
                    .build()
                    .unwrap(),
            )
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
