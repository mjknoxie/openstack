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

//! Update an existing listener.
//!
//! If the request is valid, the service returns the `Accepted (202)` response
//! code. To confirm the update, check that the listener provisioning status is
//! `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll
//! the listener object for changes.
//!
//! This operation returns the updated listener object with the `ACTIVE`,
//! `PENDING_UPDATE`, or `ERROR` provisioning status.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum ClientAuthentication {
    #[serde(rename = "MANDATORY")]
    Mandatory,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OPTIONAL")]
    Optional,
}

/// Defines attributes that are acceptable of a PUT request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Listener<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// A list of IPv4, IPv6 or mix of both CIDRs. The default is all allowed.
    /// When a list of CIDRs is provided, the default switches to deny all.
    ///
    /// **New in version 2.12**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) allowed_cidrs: Option<Vec<Cow<'a, str>>>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.20**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) alpn_protocols: Option<Vec<Cow<'a, str>>>,

    /// The TLS client authentication mode. One of the options `NONE`,
    /// `OPTIONAL` or `MANDATORY`.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) client_authentication: Option<ClientAuthentication>,

    /// The ref of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format client CA certificate bundle for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) client_ca_tls_container_ref: Option<Cow<'a, str>>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `TERMINATED_HTTPS` listeners.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) client_crl_container_ref: Option<Cow<'a, str>>,

    /// The maximum number of connections permitted for this listener. Default
    /// value is -1 which represents infinite connections or a default value
    /// defined by the provider driver.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) connection_limit: Option<i32>,

    /// The ID of the pool used by the listener if no L7 policies match. The
    /// pool has some restrictions. See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) default_pool_id: Option<Cow<'a, str>>,

    /// The URI of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `TERMINATED_HTTPS` listeners. DEPRECATED: A secret container of type
    /// “certificate” containing the certificate and key for `TERMINATED_HTTPS`
    /// listeners.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) default_tls_container_ref: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// Defines whether the `includeSubDomains` directive should be added to
    /// the Strict-Transport-Security HTTP response header. This requires
    /// setting the `hsts_max_age` option as well in order to become effective.
    ///
    /// **New in version 2.27**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) hsts_include_subdomains: Option<bool>,

    /// The value of the `max_age` directive for the Strict-Transport-Security
    /// HTTP response header. Setting this enables HTTP Strict Transport
    /// Security (HSTS) for the TLS-terminated listener.
    ///
    /// **New in version 2.27**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) hsts_max_age: Option<i32>,

    /// Defines whether the `preload` directive should be added to the
    /// Strict-Transport-Security HTTP response header. This requires setting
    /// the `hsts_max_age` option as well in order to become effective.
    ///
    /// **New in version 2.27**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) hsts_preload: Option<bool>,

    /// A dictionary of optional headers to insert into the request before it
    /// is sent to the backend `member`. See
    /// [Supported HTTP Header Insertions](#header-insertions). Both keys and
    /// values are always specified as strings.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_insert_headers"))]
    pub(crate) insert_headers: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    /// Human-readable name of the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// A list of URIs to the
    /// [key manager service](https://docs.openstack.org/barbican/latest/)
    /// secrets containing PKCS12 format certificate/key bundles for
    /// `TERMINATED_HTTPS` listeners. (DEPRECATED) Secret containers of type
    /// “certificate” containing the certificates and keys for
    /// `TERMINATED_HTTPS` listeners.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) sni_container_refs: Option<Vec<Cow<'a, str>>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    /// Frontend client inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) timeout_client_data: Option<i32>,

    /// Backend member connection timeout in milliseconds. Default: 5000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) timeout_member_connect: Option<i32>,

    /// Backend member inactivity timeout in milliseconds. Default: 50000.
    ///
    /// **New in version 2.1**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) timeout_member_data: Option<i32>,

    /// Time, in milliseconds, to wait for additional TCP packets for content
    /// inspection. Default: 0.
    ///
    /// **New in version 2.1**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) timeout_tcp_inspect: Option<i32>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tls_ciphers: Option<Cow<'a, str>>,

    /// A list of TLS protocol versions. Available versions: SSLv3, TLSv1,
    /// TLSv1.1, TLSv1.2, TLSv1.3
    ///
    /// **New in version 2.17**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tls_versions: Option<Vec<Cow<'a, str>>>,
}

impl<'a> ListenerBuilder<'a> {
    /// A dictionary of optional headers to insert into the request before it
    /// is sent to the backend `member`. See
    /// [Supported HTTP Header Insertions](#header-insertions). Both keys and
    /// values are always specified as strings.
    ///
    pub fn insert_headers<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.insert_headers
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines attributes that are acceptable of a PUT request.
    ///
    #[builder(setter(into))]
    pub(crate) listener: Listener<'a>,

    /// listener_id parameter for /v2/lbaas/listeners/{listener_id} API
    ///
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

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
    /// Add a single header to the Listener.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2/lbaas/listeners/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("listener", serde_json::to_value(&self.listener)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("listener".into())
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
                .listener(ListenerBuilder::default().build().unwrap())
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
                .listener(ListenerBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "listener"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/lbaas/listeners/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "listener": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .listener(ListenerBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/lbaas/listeners/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "listener": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .listener(ListenerBuilder::default().build().unwrap())
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