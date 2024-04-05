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
//! Wraps invoking of the `v2/lbaas/pools/{pool_id}/members` with `GET` method

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

use openstack_sdk::api::load_balancer::v2::pool::member::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists all members for the project.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and column selection](#filtering).
///
/// Administrative users can specify a project ID that is different than their
/// own to list members for other projects.
///
/// The list might be empty.
///
#[derive(Args)]
#[command(about = "List Members")]
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
    /// pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_pool_id",
        value_name = "POOL_ID"
    )]
    pool_id: String,
}
/// Members response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The IP address of the backend member server.
    ///
    #[serde()]
    #[structable(optional, wide)]
    address: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<bool>,

    /// Is the member a backup? Backup members only receive traffic when all
    /// non-backup members are down.
    ///
    /// **New in version 2.1**
    ///
    #[serde()]
    #[structable(optional, wide)]
    backup: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The ID of the member.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// An alternate IP address used for health monitoring a backend member.
    /// Default is `null` which monitors the member `address`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    monitor_address: Option<String>,

    /// An alternate protocol port used for health monitoring a backend member.
    /// Default is `null` which monitors the member `protocol_port`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    monitor_port: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional, wide)]
    operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The protocol port number the backend member server is listening on.
    ///
    #[serde()]
    #[structable(optional, wide)]
    protocol_port: Option<i32>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional, wide)]
    provisioning_status: Option<String>,

    /// The subnet ID the member service is accessible from.
    ///
    #[serde()]
    #[structable(optional, wide)]
    subnet_id: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The weight of a member determines the portion of requests or
    /// connections it services compared to the other members of the pool. For
    /// example, a member with a weight of 10 receives five times as many
    /// requests as a member with a weight of 2. A value of 0 means the member
    /// does not receive new connections but continues to service existing
    /// connections. A valid value is from `0` to `256`. Default is `1`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    weight: Option<i32>,
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
        ep_builder.pool_id(&self.path.pool_id);
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
