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

//! List Tasks command
//!
//! Wraps invoking of the `v2/images/{image_id}/tasks` with `GET` method

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

use openstack_sdk::api::image::v2::image::task::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows tasks associated with an image. *(Since Image API v2.12)*
///
/// The response body contains list of tasks, possibly empty, associated with
/// the specified image.
///
/// Preconditions
///
/// Normal response codes: 200
///
/// Error response codes: 404
///
#[derive(Args)]
#[command(about = "Show tasks associated with image")]
pub struct TasksCommand {
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
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    ///
    #[arg(id = "path_param_image_id", value_name = "IMAGE_ID")]
    image_id: String,
}
/// Tasks response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Datetime when this resource was created
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Datetime when this resource would be subject to removal
    ///
    #[serde()]
    #[structable(optional, wide)]
    expires_at: Option<String>,

    /// An identifier for the task
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Image associated with the task
    ///
    #[serde()]
    #[structable(optional, wide)]
    image_id: Option<String>,

    /// The parameters required by task, JSON blob
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    input: Option<Value>,

    /// Human-readable informative message only included when appropriate
    /// (usually on failure)
    ///
    #[serde()]
    #[structable(optional, wide)]
    message: Option<String>,

    /// An identifier for the owner of this task
    ///
    #[serde()]
    #[structable(optional, wide)]
    owner: Option<String>,

    /// Human-readable informative request-id
    ///
    #[serde()]
    #[structable(optional, wide)]
    request_id: Option<String>,

    /// The result of current task, JSON blob
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    result: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    schema: Option<String>,

    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    /// The current status of this task
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The type of task represented by this content
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,

    /// Datetime when this resource was updated
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// User associated with the task
    ///
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,
}

impl TasksCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Tasks");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.path.image_id);
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
