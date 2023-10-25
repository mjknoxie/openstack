//! Lists all Block Storage volumes, with details, that the project can access,
//! since v3.31 if non-admin users specify invalid filters in the url, API will
//! return bad request.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for volumes.get_detail operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Volumes<'a> {
    /// The UUID of the project in a multi-tenancy cloud.
    #[builder(default, setter(into))]
    project_id: Cow<'a, str>,

    /// all_projects filter parameter
    #[builder(default)]
    all_projects: Option<bool>,

    /// Name filter
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Volumes<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VolumesBuilder<'a> {
        VolumesBuilder::default()
    }
}

impl<'a> VolumesBuilder<'a> {
    /// Add a single header to the Volumes.
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

impl<'a> RestEndpoint for Volumes<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "{project_id}/volumes/detail",
            project_id = self.project_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("all_tenants", self.all_projects);
        params.push_opt("name", self.name.as_ref());

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
impl<'a> Pageable for Volumes<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Volumes::builder().build().unwrap().service_type(),
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Volumes::builder().build().unwrap().response_key().unwrap(),
            "volumes"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!(
                "/{project_id}/volumes/detail",
                project_id = "project_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumes": {} }));
        });

        let endpoint = Volumes::builder().project_id("project_id").build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!(
                    "/{project_id}/volumes/detail",
                    project_id = "project_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumes": {} }));
        });

        let endpoint = Volumes::builder()
            .project_id("project_id")
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
