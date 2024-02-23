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

//! Signals the Image Service to complete the image import workflow by
//! processing data that has been made available to the OpenStack image
//! service. *(Since Image API v2.6)*
//!
//! In the `glance-direct` workflow, the data has been made available to the
//! Image service via the [Stage binary image data](#image-stage-call) API
//! call.
//!
//! In the `web-download` workflow, the data is made available to the Image
//! service by being posted to an accessible location with a URL that you know.
//!
//! In the `copy-image` workflow, the data is made available to the Image
//! service by copying existing image data to the staging area.
//!
//! In the `glance-download` workflow, the data is made available to the Image
//! service by fetching an image accessible from another glance service
//! specified by a region name and an image id that you know.
//!
//! Beginning with API version 2.8, an optional `stores` parameter may be added
//! to the body request. When present, it contains the list of backing store
//! identifiers to import the image binary data to. If at least one store
//! identifier specified is not recognized, a 409 (Conflict) response is
//! returned. When the parameter is not present, the image data is placed into
//! the default backing store.
//!
//! For backwards compatibility, if the `stores` parameter is not specified,
//! the header ‘X-Image-Meta-Store’ is evaluated.
//!
//! To import the data into the entire set of stores you may consume from this
//! particular deployment of Glance without specifying each one of them, you
//! can use the optional boolean body parameter `all_stores`. Note that this
//! can’t be used simultaneously with the `stores` parameter.
//!
//! To set the behavior of the import workflow in case of error, you can use
//! the optional boolean body parameter `all_stores_must_succeed`. When set to
//! True (default), if an error occurs during the upload in at least one store,
//! the workflow fails, the data is deleted from stores where copying is done
//! and the state of the image remains unchanged. When set to False, the
//! workflow will fail only if the upload fails on all stores specified. In
//! case of a partial success, the locations added to the image will be the
//! stores where the data has been correctly uploaded.
//!
//! The JSON request body specifies what import method you wish to use for this
//! image request.
//!
//! **Preconditions**
//!
//! Before you can complete the interoperable image import workflow, you must
//! meet the following preconditions:
//!
//! **Additional Preconditions**
//!
//! If you are using the `glance-direct` import method:
//!
//! If you are using the `web-download` import method:
//!
//! If you are using the `copy-image` import method:
//!
//! If you are using the `glance-download` import method:
//!
//! **Synchronous Postconditions**
//!
//! Normal response codes: 202
//!
//! Error response codes: 400, 401, 403, 404, 405, 409, 410, 413, 415, 503
//!
//! If the image import process is not enabled in your cloud, this request will
//! result in a 404 response code with an appropriate message.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// A JSON object indicating what import method you wish to use to import your
/// image. The content of this JSON object is another JSON object with a `name`
/// field whose value is the identifier for the import method.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Method<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) uri: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) glance_image_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) glance_region: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) glance_service_interface: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A JSON object indicating what import method you wish to use to import
    /// your image. The content of this JSON object is another JSON object with
    /// a `name` field whose value is the identifier for the import method.
    ///
    #[builder(default, setter(into))]
    pub(crate) method: Option<Method<'a>>,

    /// If present contains the list of store id to import the image binary
    /// data to.
    ///
    #[builder(default, setter(into))]
    pub(crate) stores: Option<Vec<Cow<'a, str>>>,

    /// When set to True the data will be imported to the set of stores you may
    /// consume from this particular deployment of Glance (ie: the same set of
    /// stores returned to a call to /v2/info/stores on the glance-api the
    /// request hits). This can’t be used simultaneously with the `stores`
    /// parameter.
    ///
    #[builder(default)]
    pub(crate) all_stores: Option<bool>,

    #[builder(default)]
    pub(crate) all_stores_must_success: Option<bool>,

    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    ///
    #[builder(default, setter(into))]
    image_id: Cow<'a, str>,

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
    /// Add a single header to the Import.
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
            "v2/images/{image_id}/import",
            image_id = self.image_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        if let Some(val) = &self.method {
            params.push("method", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.stores {
            params.push("stores", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.all_stores {
            params.push("all_stores", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.all_stores_must_success {
            params.push("all_stores_must_success", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
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
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2/images/{image_id}/import",
                image_id = "image_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder().image_id("image_id").build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!(
                    "/v2/images/{image_id}/import",
                    image_id = "image_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .image_id("image_id")
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
