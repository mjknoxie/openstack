//! Shows details for an account and lists containers, sorted by name, in the
//! account.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for account.get operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Account<'a> {
    /// For an integer value n, limits the number of results to n.
    #[builder(default)]
    limit: Option<u32>,

    /// For a string value, x, constrains the list to items whose names are
    /// greater than x.
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// For a string value, x, constrains the list to items whose names are
    /// less than x.
    #[builder(default, setter(into))]
    end_marker: Option<Cow<'a, str>>,

    /// The response format. Valid values are json, xml, or plain. The default
    /// is plain. If you append the format=xml or format=json query parameter
    /// to the storage account URL, the response shows extended container
    /// information serialized in that format. If you append the format=plain
    /// query parameter, the response lists the container names separated by
    /// newlines.
    #[builder(default, setter(into))]
    format: Option<Cow<'a, str>>,

    /// Only objects with this prefix will be returned. When combined with a
    /// delimiter query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[builder(default, setter(into))]
    prefix: Option<Cow<'a, str>>,

    /// The delimiter is a single character used to split object names to
    /// present a pseudo-directory hierarchy of objects. When combined with a
    /// prefix query, this enables API users to simulate and traverse the
    /// objects in a container as if they were in a directory tree.
    #[builder(default, setter(into))]
    delimiter: Option<Cow<'a, str>>,

    /// By default, listings are returned sorted by name, ascending. If you
    /// include the reverse=true query parameter, the listing will be returned
    /// sorted by name, descending.
    #[builder(default)]
    reverse: Option<bool>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Account<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AccountBuilder<'a> {
        AccountBuilder::default()
    }
}

impl<'a> AccountBuilder<'a> {
    /// Add a single header to the Account.
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

impl<'a> RestEndpoint for Account<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        String::new().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("end_marker", self.end_marker.as_ref());
        params.push_opt("format", self.format.as_ref());
        params.push_opt("prefix", self.prefix.as_ref());
        params.push_opt("delimiter", self.delimiter.as_ref());
        params.push_opt("reverse", self.reverse);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::ObjectStore
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Account<'a> {}

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
            Account::builder().build().unwrap().service_type(),
            ServiceType::ObjectStore
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Account::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!("/",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Account::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Account::builder()
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
