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

//! List ApplicationCredentials command
//!
//! Wraps invoking of the `v3/users/{user_id}/application_credentials` with `GET` method

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

use openstack_sdk::api::identity::v3::user::application_credential::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// List all application credentials for a user.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/application\_credentials`
#[derive(Args)]
#[command(about = "List application credentials")]
pub struct ApplicationCredentialsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// The name of the application credential. Must be unique to a user.
    #[arg(long)]
    name: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg(value_name = "USER_ID", id = "path_param_user_id")]
    user_id: String,
}
/// ApplicationCredentials response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the application credential.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    roles: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    unrestricted: Option<bool>,

    #[serde()]
    #[structable(optional, wide)]
    access_rules: Option<Value>,
}

impl ApplicationCredentialsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ApplicationCredentials");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.path.user_id);
        // Set query parameters
        if let Some(val) = &self.query.name {
            ep_builder.name(val.clone());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
