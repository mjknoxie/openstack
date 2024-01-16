//! Uploads binary image data.
//! *(Since Image API v2.0)*
//!
//! Set the `Content-Type` request header to `application/octet-stream`.
//!
//! A multiple store backend support is introduced in the Rocky release
//! as a part of the EXPERIMENTAL Image API v2.8.
//!
//! Beginning with API version 2.8, an optional `X-Image-Meta-Store`
//! header may be added to the request. When present, the image data will be
//! placed into the backing store whose identifier is the value of this
//! header. If the store identifier specified is not recognized, a 400 (Bad
//! Request) response is returned. When the header is not present, the image
//! data is placed into the default backing store.
//!
//! Example call:
//!
//! **Preconditions**
//!
//! Before you can store binary image data, you must meet the following
//! preconditions:
//!
//! **Synchronous Postconditions**
//!
//! **Troubleshooting**
//!
//! Normal response codes: 204
//!
//! Error response codes: 400, 401, 403, 404, 409, 410, 413, 415, 503
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[builder(default, setter(into))]
    image_id: Cow<'a, str>,

    /// The media type descriptor of the body, namely application/octet-stream
    #[builder(default, setter(into))]
    content_type: Option<Cow<'a, str>>,

    /// A store identifier to upload or import image data. Should only be
    /// included when making a request to a cloud that supports multiple
    /// backing stores. Use the Store Discovery call to determine an
    /// appropriate store identifier. Simply omit this header to use the
    /// default store.
    #[builder(default, setter(into))]
    x_image_meta_store: Option<Cow<'a, str>>,

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
    /// Add a single header to the File.
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
        format!(
            "v2/images/{image_id}/file",
            image_id = self.image_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
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
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/images/{image_id}/file", image_id = "image_id",));

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
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/images/{image_id}/file", image_id = "image_id",))
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