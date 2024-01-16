//! Creates a catalog record for an operating system disk image.
//! *(Since Image API v2.0)*
//!
//! The `Location` response header contains the URI for the image.
//!
//! A multiple store backend support is introduced in the Rocky release
//! as a part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a
//! new header `OpenStack-image-store-ids` which contains the list of
//! available stores will be included in response. This header is only
//! included if multiple backend stores are supported.
//!
//! The response body contains the new image entity.
//!
//! Synchronous Postconditions
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 409, 413, 415
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Visibility {
    #[serde(rename = "community")]
    Community,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "shared")]
    Shared,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum ContainerFormat {
    #[serde(rename = "aki")]
    Aki,
    #[serde(rename = "ami")]
    Ami,
    #[serde(rename = "ari")]
    Ari,
    #[serde(rename = "bare")]
    Bare,
    #[serde(rename = "compressed")]
    Compressed,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "ova")]
    Ova,
    #[serde(rename = "ovf")]
    Ovf,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum DiskFormat {
    #[serde(rename = "aki")]
    Aki,
    #[serde(rename = "ami")]
    Ami,
    #[serde(rename = "ari")]
    Ari,
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "ploop")]
    Ploop,
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vdi")]
    Vdi,
    #[serde(rename = "vhd")]
    Vhd,
    #[serde(rename = "vhdx")]
    Vhdx,
    #[serde(rename = "vmdk")]
    Vmdk,
}

/// Values to be used to populate the corresponding image properties. If the
/// image status is not 'queued', values must exactly match those already
/// contained in the image properties.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ValidationData<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    checksum: Option<Cow<'a, str>>,

    #[serde()]
    #[builder(setter(into))]
    os_hash_algo: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    os_hash_value: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Locations<'a> {
    #[serde()]
    #[builder(setter(into))]
    url: Cow<'a, str>,

    #[serde()]
    #[builder(private, setter(name = "_metadata"))]
    metadata: BTreeMap<Cow<'a, str>, Value>,

    /// Values to be used to populate the corresponding image properties. If
    /// the image status is not 'queued', values must exactly match those
    /// already contained in the image properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    validation_data: Option<ValidationData<'a>>,
}

impl<'a> LocationsBuilder<'a> {
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self.metadata
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A unique, user-defined image UUID, in the format:
    ///
    ///
    ///
    /// ```text
    /// nnnnnnnn-nnnn-nnnn-nnnn-nnnnnnnnnnnn
    ///
    /// ```
    ///
    ///
    /// Where **n** is a hexadecimal digit from 0 to f, or F.
    ///
    ///
    /// For example:
    ///
    ///
    ///
    /// ```text
    /// b2173dd3-7ad6-4362-baa6-a68bce3565cb
    ///
    /// ```
    ///
    ///
    /// If you omit this value, the API generates a UUID for the image. If you
    /// specify a value that has already been assigned, the request fails with
    /// a `409` response code.
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// The name of the image.
    #[builder(default, setter(into))]
    name: Option<Option<Cow<'a, str>>>,

    /// Visibility for this image. Valid value is one of: `public`, `private`,
    /// `shared`, or `community`.
    /// At most sites, only an administrator can make an image `public`.
    /// Some sites may restrict what users can make an image `community`.
    /// Some sites may restrict what users can perform member operations on
    /// a `shared` image.
    /// *Since the Image API v2.5, the default value is ``shared``.*
    #[builder(default)]
    visibility: Option<Visibility>,

    /// Image protection for deletion. Valid value is `true` or `false`.
    /// Default is `false`.
    #[builder(default)]
    protected: Option<bool>,

    /// If true, image will not appear in default image list response.
    #[builder(default)]
    os_hidden: Option<bool>,

    /// Owner of the image
    #[builder(default, setter(into))]
    owner: Option<Option<Cow<'a, str>>>,

    /// Format of the image container.
    ///
    ///
    /// Values may vary based on the configuration available in a
    /// particular OpenStack cloud. See the [Image Schema](#image-schema)
    /// response from the cloud itself for the valid values available.
    ///
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `bare`,
    /// `ovf`, `ova`, or `docker`.
    ///
    ///
    /// The value might be `null` (JSON null data type).
    #[builder(default)]
    container_format: Option<ContainerFormat>,

    /// The format of the disk.
    ///
    ///
    /// Values may vary based on the configuration available in a
    /// particular OpenStack cloud. See the [Image Schema](#image-schema)
    /// response from the cloud itself for the valid values available.
    ///
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `vhd`,
    /// `vhdx`, `vmdk`, `raw`, `qcow2`, `vdi`, `ploop` or
    /// `iso`.
    ///
    ///
    /// The value might be `null` (JSON null data type).
    ///
    ///
    /// **Newton changes**: The `vhdx` disk format is a supported
    /// value.
    ///
    /// **Ocata changes**: The `ploop` disk format is a supported
    /// value.
    #[builder(default)]
    disk_format: Option<DiskFormat>,

    /// List of tags for this image. Each tag is a string of at most 255 chars.
    /// The maximum number of tags allowed on an image is set by the operator.
    #[builder(default, setter(into))]
    tags: Option<Vec<Cow<'a, str>>>,

    /// Amount of RAM in MB that is required to boot the image.
    #[builder(default)]
    min_ram: Option<i32>,

    /// Amount of disk space in GB that is required to boot the image.
    #[builder(default)]
    min_disk: Option<i32>,

    /// A set of URLs to access the image file kept in external store
    #[builder(default, setter(into))]
    locations: Option<Vec<Locations<'a>>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
    #[builder(setter(name = "_properties"), default, private)]
    _properties: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
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

    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v2/images".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        if let Some(val) = &self.id {
            params.push("id", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.name {
            params.push("name", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.visibility {
            params.push("visibility", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.protected {
            params.push("protected", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.os_hidden {
            params.push("os_hidden", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.owner {
            params.push("owner", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.container_format {
            params.push("container_format", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.disk_format {
            params.push("disk_format", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.tags {
            params.push("tags", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.min_ram {
            params.push("min_ram", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.min_disk {
            params.push("min_disk", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.locations {
            params.push("locations", serde_json::to_value(val)?);
        }
        for (key, val) in &self._properties {
            params.push(key.clone(), serde_json::Value::from(val.clone()));
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
            when.method(httpmock::Method::POST)
                .path(format!("/v2/images",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2/images",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
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