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

//! Show ApplicationCredential command
//!
//! Wraps invoking of the `v3/users/{user_id}/application_credentials/{application_credential_id}` with `GET` method

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

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::user::application_credential::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Show details of an application credential.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`
///
#[derive(Args)]
#[command(about = "Show application credential details")]
pub struct ApplicationCredentialCommand {
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
    ///
    #[arg(id = "path_param_user_id", value_name = "USER_ID")]
    user_id: String,

    /// application_credential_id parameter for
    /// /v3/users/{user_id}/application_credentials/{application_credential_id}
    /// API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// ApplicationCredential response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the application credential.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    roles: Option<Value>,

    #[serde()]
    #[structable(optional)]
    unrestricted: Option<bool>,

    #[serde()]
    #[structable(optional, pretty)]
    access_rules: Option<Value>,
}

impl ApplicationCredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ApplicationCredential");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.user_id(&self.path.user_id);
        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
