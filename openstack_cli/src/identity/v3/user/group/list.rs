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

//! List Groups command
//!
//! Wraps invoking of the `v3/users/{user_id}/groups` with `GET` method

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

use openstack_sdk::api::identity::v3::user::group::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists groups to which a user belongs.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/user\_groups`
#[derive(Args)]
#[command(about = "List groups to which a user belongs")]
pub struct GroupsCommand {
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
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg(value_name = "USER_ID", id = "path_param_user_id")]
    user_id: String,
}
/// Groups response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the group.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain of the group.
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// The ID of the group.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the group.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The date and time when the group membership expires.
    /// A `null` value indicates that the membership never expires.
    ///
    ///
    /// **New in version 3.14**
    #[serde()]
    #[structable(optional, wide)]
    membership_expires_at: Option<String>,
}

impl GroupsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Groups");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.path.user_id);
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
