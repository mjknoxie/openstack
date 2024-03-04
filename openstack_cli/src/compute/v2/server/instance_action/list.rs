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

//! List InstanceActions command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-instance-actions` with `GET` method

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

use openstack_sdk::api::compute::v2::server::instance_action::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Lists actions for a server.
///
/// Action information of deleted instances can be returned for requests
/// starting with microversion 2.21.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
///
#[derive(Args)]
#[command(about = "List Actions For Server")]
pub struct InstanceActionsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,

    #[arg(long)]
    changes_since: Option<String>,

    #[arg(long)]
    changes_before: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    ///
    #[arg(id = "path_param_server_id", value_name = "SERVER_ID")]
    server_id: String,
}
/// InstanceActions response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name of the action.
    ///
    #[serde()]
    #[structable(optional)]
    action: Option<String>,

    /// The events which occurred in this action in descending order of
    /// creation.
    ///
    /// Policy defaults enable only users with the administrative role or the
    /// owner of the server to see instance action event information. Cloud
    /// providers can change these permissions through the `policy.json` file.
    ///
    /// **New in version 2.51**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    events: Option<Value>,

    /// The related error message for when an action fails.
    ///
    #[serde()]
    #[structable(optional)]
    message: Option<String>,

    /// The ID of the project which initiated the server action.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The request id generated when execute the API of this action.
    ///
    #[serde()]
    #[structable(optional)]
    request_id: Option<String>,

    /// The date and time when the action was started.
    ///
    #[serde()]
    #[structable(optional)]
    start_time: Option<String>,

    /// The ID of the user which initiated the server action.
    ///
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// The date and time when the instance action or the action event of
    /// instance action was updated. The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    /// **New in version 2.58**
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl InstanceActionsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List InstanceActions");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.changes_since {
            ep_builder.changes_since(val);
        }
        if let Some(val) = &self.query.changes_before {
            ep_builder.changes_before(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
