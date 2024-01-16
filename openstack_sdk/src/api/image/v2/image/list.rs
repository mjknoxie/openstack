//! Lists public virtual machine (VM) images.
//! *(Since Image API v2.0)*
//!
//! **Pagination**
//!
//! Returns a subset of the larger collection of images and a link that you can
//! use
//! to get the next set of images. You should always check for the presence of
//! a
//! `next` link and use it as the URI in a subsequent HTTP GET request. You
//! should follow this pattern until a `next` link is no longer provided.
//!
//! The `next` link preserves any query parameters that you send in your
//! initial
//! request. You can use the `first` link to jump back to the first page of the
//! collection. If you prefer to paginate through images manually, use the
//! `limit` and `marker` parameters.
//!
//! **Query Filters**
//!
//! The list operation accepts query parameters to filter the response.
//!
//! A client can provide direct comparison filters by using most image
//! attributes,
//! such as `name=Ubuntu`, `visibility=public`, and so on.
//!
//! To filter using image tags, use the filter `tag` (note the singular). To
//! filter on multiple tags, include each tag separately in the query. For
//! example, to find images with the tag **ready**, include `tag=ready` in your
//! query string. To find images tagged with **ready** and **approved**,
//! include
//! `tag=ready&tag=approved` in your query string. (Note that only images
//! containing *both* tags will be included in the response.)
//!
//! A client cannot use any `link` in the json-schema, such as self, file, or
//! schema, to filter the response.
//!
//! You can list VM images that have a status of `active`, `queued`, or
//! `saving`.
//!
//! **The** `in` **Operator**
//!
//! As a convenience, you may specify several values for any of the following
//! fields by using the `in` operator:
//!
//! For most of these, usage is straight forward. For example, to list images
//! in queued or saving status, use:
//!
//! `GET /v2/images?status=in:saving,queued`
//!
//! To find images in a particular list of image IDs, use:
//!
//! `GET /v2/images?id=in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-
//! 44b5-baf2-2eb4babae53d`
//!
//! Using the `in` operator with the `name` property of images can be a bit
//! trickier, depending upon how creatively you have named your images. The
//! general rule is that if an image name contains a comma (`,`), you must
//! enclose the entire name in quotation marks (`"`). As usual, you must URL
//! encode any characters that require it.
//!
//! For example, to find images named `glass, darkly` or `share me`, you would
//! use the following filter specification:
//!
//! `GET v2/images?name=in:"glass,%20darkly",share%20me`
//!
//! As with regular filtering by name, you must specify the complete name you
//! are
//! looking for. Thus, for example, the query string `name=in:glass,share` will
//! only match images with the exact name `glass` or the exact name `share`.
//! It will not find an image named `glass, darkly` or an image named `share
//! me`.
//!
//! **Size Comparison Filters**
//!
//! You can use the `size\_min` and `size\_max` query parameters to filter
//! images
//! that are greater than or less than the image size. The size, in bytes, is
//! the
//! size of an image on disk.
//!
//! For example, to filter the container to include only images that are from 1
//! to
//! 4 MB, set the `size\_min` query parameter to `1048576` and the `size\_max`
//! query parameter to `4194304`.
//!
//! **Time Comparison Filters**
//!
//! You can use a *comparison operator* along with the `created\_at` or
//! `updated\_at` fields to filter your results. Specify the operator first, a
//! colon (`:`) as a separator, and then the time in [ISO 8601
//! Format](https://en.wikipedia.org/wiki/ISO_8601). Available comparison
//! operators
//! are:
//!
//! For example:
//!
//! **Sorting**
//!
//! You can use query parameters to sort the results of this operation.
//!
//! To sort the response, use the `sort\_key` and `sort\_dir` query
//! parameters:
//!
//! Alternatively, specify the `sort` query parameter:
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[builder(default)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// Filters the response by a name, as a string. A valid value is the name
    /// of an image.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// id filter parameter
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// Filters the response by a project (also called a “tenant”) ID. Shows
    /// only images that are shared with you by the specified owner.
    #[builder(default, setter(into))]
    owner: Option<Cow<'a, str>>,

    /// Filters the response by the ‘protected’ image property. A valid value
    /// is one of ‘true’, ‘false’ (must be all lowercase). Any other value will
    /// result in a 400 response.
    #[builder(default)]
    protected: Option<bool>,

    /// Filters the response by an image status.
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// Filters the response by the specified tag value. May be repeated, but
    /// keep in mind that you’re making a conjunctive query, so only images
    /// containing all the tags specified will appear in the response.
    #[builder(default, private, setter(name = "_tag"))]
    tag: BTreeSet<Cow<'a, str>>,

    /// Filters the response by an image visibility value. A valid value is
    /// public, private, community, shared, or all. (Note that if you filter on
    /// shared, the images included in the response will only be those where
    /// your member status is accepted unless you explicitly include a
    /// member_status filter in the request.) If you omit this parameter, the
    /// response shows public, private, and those shared images with a member
    /// status of accepted.
    #[builder(default, setter(into))]
    visibility: Option<Cow<'a, str>>,

    /// When true, filters the response to display only "hidden" images. By
    /// default, "hidden" images are not included in the image-list response.
    /// (Since Image API v2.7)
    #[builder(default)]
    os_hidden: Option<bool>,

    /// Filters the response by a member status. A valid value is accepted,
    /// pending, rejected, or all. Default is accepted.
    #[builder(default, setter(into))]
    member_status: Option<Cow<'a, str>>,

    /// Filters the response by a maximum image size, in bytes.
    #[builder(default, setter(into))]
    size_max: Option<Cow<'a, str>>,

    /// Filters the response by a minimum image size, in bytes.
    #[builder(default, setter(into))]
    size_min: Option<Cow<'a, str>>,

    /// Specify a comparison filter based on the date and time when the
    /// resource was created.
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// Specify a comparison filter based on the date and time when the
    /// resource was most recently modified.
    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

    /// Sorts the response by a set of one or more sort direction and attribute
    /// (sort_key) combinations. A valid value for the sort direction is asc
    /// (ascending) or desc (descending). If you omit the sort direction in a
    /// set, the default is desc.
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// Sorts the response by an attribute, such as name, id, or updated_at.
    /// Default is created_at. The API uses the natural sorting direction of
    /// the sort_key image attribute.
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    /// Sorts the response by one or more attribute and sort direction
    /// combinations. You can also set multiple sort keys and directions.
    /// Default direction is desc. Use the comma (,) character to separate
    /// multiple values. For example: `sort=name:asc,status:desc`
    #[builder(default, setter(into))]
    sort: Option<Cow<'a, str>>,

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
    /// Filters the response by the specified tag value. May be repeated, but
    /// keep in mind that you’re making a conjunctive query, so only images
    /// containing all the tags specified will appear in the response.
    pub fn tag<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tag
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Image.
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
        "v2/images".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("id", self.id.as_ref());
        params.push_opt("owner", self.owner.as_ref());
        params.push_opt("protected", self.protected);
        params.push_opt("status", self.status.as_ref());
        params.extend(self.tag.iter().map(|value| ("tag", value)));
        params.push_opt("visibility", self.visibility.as_ref());
        params.push_opt("os_hidden", self.os_hidden);
        params.push_opt("member_status", self.member_status.as_ref());
        params.push_opt("size_max", self.size_max.as_ref());
        params.push_opt("size_min", self.size_min.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("updated_at", self.updated_at.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort", self.sort.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("images".into())
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
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "images"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2/images".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "images": {} }));
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
                .path("/v2/images".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "images": {} }));
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
