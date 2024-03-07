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

//! List Members command
//!
//! Wraps invoking of the `v2/images/{image_id}/members` with `GET` method

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

use openstack_sdk::api::image::v2::image::member::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists the tenants that share this image. *(Since Image API v2.1)*
///
/// If the image owner makes this call, the complete member list is returned.
///
/// If a user who is an image member makes this call, the member list contains
/// only information for that user.
///
/// If a user who is not an image member makes this call, the call returns the
/// HTTP `404` response code.
///
/// Preconditions
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404
///
#[derive(Args)]
#[command(about = "List image members")]
pub struct MembersCommand {
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
/// Members response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Date and time of image member creation
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// An identifier for the image
    ///
    #[serde()]
    #[structable(optional)]
    image_id: Option<String>,

    /// An identifier for the image member (tenantId)
    ///
    #[serde()]
    #[structable(optional)]
    member_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    schema: Option<String>,

    /// The status of this image member
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// Date and time of last modification of image member
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl MembersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Members");

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
