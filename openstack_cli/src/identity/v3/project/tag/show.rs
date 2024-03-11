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

//! Show Tag command
//!
//! Wraps invoking of the `v3/projects/{project_id}/tags/{value}` with `GET` method

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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::identity::v3::project::tag::get;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Checks if a project contains the specified tag.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/projects`
///
#[derive(Args)]
#[command(about = "Check if project contains tag")]
pub struct TagCommand {
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
    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_project_id",
        value_name = "PROJECT_ID"
    )]
    project_id: String,

    /// value parameter for /v3/projects/{project_id}/tags/{value} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_value",
        value_name = "VALUE"
    )]
    value: String,
}
/// Tag response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Tag");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.project_id(&self.path.project_id);
        ep_builder.value(&self.path.value);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
