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

//! Show ServiceProvider command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/service_providers/{sp_id}` with `GET` method

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

use openstack_sdk::api::identity::v3::os_federation::service_provider::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// GET operation on /v3/OS-FEDERATION/service_providers/{sp_id}
#[derive(Args)]
pub struct ServiceProviderCommand {
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
    /// sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API
    #[arg(value_name = "SP_ID", id = "path_param_sp_id")]
    sp_id: String,
}
/// ServiceProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The URL to authenticate against
    #[serde()]
    #[structable()]
    auth_url: String,

    /// The description of the Service Provider
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The Service Provider unique ID
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Whether the Service Provider is enabled or not
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The prefix of the RelayState SAML attribute
    #[serde()]
    #[structable(optional)]
    relay_state_prefix: Option<String>,

    /// The Service Provider’s URL
    #[serde()]
    #[structable()]
    sp_url: String,
}

impl ServiceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ServiceProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.sp_id(&self.path.sp_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
