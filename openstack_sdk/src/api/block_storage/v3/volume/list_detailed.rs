//! Returns a detailed list of volumes.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Shows details for all project. Admin only.
    #[builder(default)]
    all_tenans: Option<bool>,

    /// Comma-separated list of sort keys and optional sort directions in the
    /// form of < key > [: < direction > ]. A valid direction is asc
    /// (ascending) or desc (descending).
    #[builder(default, setter(into))]
    sort: Option<Cow<'a, str>>,

    /// Sorts by an attribute. A valid value is name, status, container_format,
    /// disk_format, size, id, created_at, or updated_at. Default is
    /// created_at. The API uses the natural sorting direction of the sort_key
    /// attribute value. Deprecated in favour of the combined sort parameter.
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    /// Sorts by one or more sets of attribute and sort direction combinations.
    /// If you omit the sort direction in a set, default is desc. Deprecated in
    /// favour of the combined sort parameter.
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[builder(default)]
    limit: Option<i32>,

    /// Used in conjunction with limit to return a slice of items. offset is
    /// where to start in the list.
    #[builder(default)]
    offset: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// Whether to show count in API response or not, default is False.
    #[builder(default)]
    with_count: Option<bool>,

    /// Filters reuslts by a time that resources are created at with time
    /// comparison operators: gt/gte/eq/neq/lt/lte.
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// Filters reuslts by a time that resources are updated at with time
    /// comaprison operators: gt/gte/eq/neq/lt/lte.
    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

    /// Filters results by consumes_quota field. Resources that don’t use
    /// quotas are usually temporary internal resources created to perform an
    /// operation. Default is to not filter by it. Filtering by this option may
    /// not be always possible in a cloud, see List Resource Filters to
    /// determine whether this filter is available in your cloud.
    #[builder(default)]
    consumes_quota: Option<bool>,

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
    /// Add a single header to the Volume.
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
        "v3/volumes/detail".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("all_tenans", self.all_tenans);
        params.push_opt("sort", self.sort.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("offset", self.offset);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("with_count", self.with_count);
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("updated_at", self.updated_at.as_ref());
        params.push_opt("consumes_quota", self.consumes_quota);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::BlockStorage
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("volumes".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Request<'a> {}

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
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "volumes"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/v3/volumes/detail",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumes": {} }));
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
                .path(format!("/v3/volumes/detail",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumes": {} }));
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