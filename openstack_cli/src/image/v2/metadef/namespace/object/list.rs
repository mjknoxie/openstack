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

//! List Objects command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/objects` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::image::v2::metadef::namespace::object::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Lists object definitions in a namespace.
///
/// Returns a subset of the larger collection of namespaces and a link that you
/// can use to get the next set of namespaces. You should always check for the
/// presence of a `next` link and use it as the URI in a subsequent HTTP GET
/// request. You should follow this pattern until a `next` link is no longer
/// provided. The next link preserves any query parameters that you send in
/// your initial request. You can use the `first` link to jump back to the
/// first page of the collection. If you prefer to paginate through namespaces
/// manually, use the `limit` and `marker` parameters.
///
/// Use the `resource_types` and `visibility` query parameters to filter the
/// response.
///
/// For example, set the `resource_types` query parameter to
/// `OS::Glance::Image,OS::Nova::Flavor` to filter the response to include only
/// namespaces that are associated with the given resource types.
///
/// You can sort the results of this operation by using the `sort_key` and
/// `sort_dir` parameters. The API uses the natural sorting of whatever
/// namespace attribute is provided as the `sort_key`.
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403, 404
///
#[derive(Args)]
#[command(about = "List objects")]
pub struct ObjectsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,
}
/// Objects response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Date and time of object creation
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable()]
    name: String,

    #[serde()]
    #[structable(optional, pretty)]
    properties: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    required: Option<Value>,

    #[serde()]
    #[structable(optional)]
    schema: Option<String>,

    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    /// Date and time of the last object modification
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseProperties {
    additional_items: Option<bool>,
    _default: Option<Value>,
    description: Option<String>,
    _enum: Option<Value>,
    items: Option<Value>,
    maximum: Option<f32>,
    max_items: Option<i32>,
    max_length: Option<i32>,
    minimum: Option<f32>,
    min_items: Option<i32>,
    min_length: Option<i32>,
    name: Option<String>,
    operators: Option<Value>,
    pattern: Option<String>,
    readonly: Option<bool>,
    required: Option<Value>,
    title: String,
    _type: String,
    unique_items: Option<bool>,
}

impl fmt::Display for ResponseProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "additional_items={}",
                self.additional_items
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "_default={}",
                self._default
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "description={}",
                self.description
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "_enum={}",
                self._enum
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "items={}",
                self.items
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "maximum={}",
                self.maximum
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "max_items={}",
                self.max_items
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "max_length={}",
                self.max_length
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "minimum={}",
                self.minimum
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "min_items={}",
                self.min_items
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "min_length={}",
                self.min_length
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "operators={}",
                self.operators
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "pattern={}",
                self.pattern
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "readonly={}",
                self.readonly
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "required={}",
                self.required
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!("title={}", self.title),
            format!("_type={}", self._type),
            format!(
                "unique_items={}",
                self.unique_items
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl ObjectsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Objects");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
