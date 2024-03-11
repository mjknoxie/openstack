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

//! List FlavorAccesses command
//!
//! Wraps invoking of the `v2.1/flavors/{flavor_id}/os-flavor-access` with `GET` method

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

use openstack_sdk::api::compute::v2::flavor::flavor_access::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists flavor access information.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
///
#[derive(Args)]
#[command(about = "List Flavor Access Information For Given Flavor")]
pub struct FlavorAccessesCommand {
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
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_flavor_id",
        value_name = "FLAVOR_ID"
    )]
    flavor_id: String,
}
/// FlavorAccesses response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,
}

impl FlavorAccessesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List FlavorAccesses");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.flavor_id(&self.path.flavor_id);
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
