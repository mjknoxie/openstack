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

//! List Domains command
//!
//! Wraps invoking of the `v3/auth/domains` with `GET` method

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

use openstack_sdk::api::identity::v3::auth::domain::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// New in version 3.3
///
/// This call returns the list of domains that are available to be scoped to
/// based on the X-Auth-Token provided in the request.
///
/// The structure is the same as listing domains.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/auth_domains`
///
#[derive(Args)]
#[command(about = "Get available domain scopes")]
pub struct DomainsCommand {
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
struct PathParameters {}
/// Domains response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The description of the domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// The links to the `domain` resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    links: Option<Value>,
}

impl DomainsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Domains");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
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
