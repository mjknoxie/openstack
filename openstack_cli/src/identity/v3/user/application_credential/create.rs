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

//! Create ApplicationCredential command
//!
//! Wraps invoking of the `v3/users/{user_id}/application_credentials` with `POST` method

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

use crate::common::parse_json;

use openstack_sdk::api::identity::v3::user::application_credential::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates an application credential for a user on the project to which the
/// current token is scoped.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`
///
#[derive(Args)]
#[command(about = "Create application credential")]
pub struct ApplicationCredentialCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    application_credential: ApplicationCredential,
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
}
/// ApplicationCredential Body data
#[derive(Args)]
struct ApplicationCredential {
    /// The name of the application credential. Must be unique to a user.
    ///
    #[arg(long)]
    name: String,

    /// A description of the application credential’s purpose.
    ///
    #[arg(long)]
    description: Option<String>,

    /// The secret that the application credential will be created with. If not
    /// provided, one will be generated.
    ///
    #[arg(long)]
    secret: Option<String>,

    /// An optional expiry time for the application credential. If unset, the
    /// application credential does not expire.
    ///
    #[arg(long)]
    expires_at: Option<String>,

    /// An optional list of role objects, identified by ID or name. The list
    /// may only contain roles that the user has assigned on the project. If
    /// not provided, the roles assigned to the application credential will be
    /// the same as the roles in the current token.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    roles: Option<Vec<Value>>,

    /// An optional flag to restrict whether the application credential may be
    /// used for the creation or destruction of other application credentials
    /// or trusts. Defaults to false.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    unrestricted: Option<bool>,

    /// A list of `access_rules` objects
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    access_rules: Option<Vec<Value>>,
}

/// ApplicationCredential response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The UUID for the credential.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID for the project.
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
    #[structable(optional)]
    roles: Option<Value>,

    #[serde()]
    #[structable(optional)]
    unrestricted: Option<bool>,

    #[serde()]
    #[structable(optional)]
    access_rules: Option<Value>,

    /// The secret for the application credential, either generated by the
    /// server or provided by the user. This is only ever shown once in the
    /// response to a create request. It is not stored nor ever shown again. If
    /// the secret is lost, a new application credential must be created.
    ///
    #[serde()]
    #[structable(optional)]
    secret: Option<String>,
}

impl ApplicationCredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ApplicationCredential");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.path.user_id);
        // Set query parameters
        // Set body parameters
        // Set Request.application_credential data
        let args = &self.application_credential;
        let mut application_credential_builder = create::ApplicationCredentialBuilder::default();

        application_credential_builder.name(args.name.clone());

        if let Some(val) = &args.description {
            application_credential_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.secret {
            application_credential_builder.secret(Some(val.into()));
        }

        if let Some(val) = &args.expires_at {
            application_credential_builder.expires_at(Some(val.into()));
        }

        if let Some(val) = &args.roles {
            let roles_builder: Vec<create::Roles> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Roles>(v.clone()))
                .collect::<Vec<create::Roles>>();
            application_credential_builder.roles(roles_builder);
        }

        if let Some(val) = &args.unrestricted {
            application_credential_builder.unrestricted(*val);
        }

        if let Some(val) = &args.access_rules {
            let access_rules_builder: Vec<create::AccessRules> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::AccessRules>(v.clone()))
                .collect::<Vec<create::AccessRules>>();
            application_credential_builder.access_rules(access_rules_builder);
        }

        ep_builder.application_credential(application_credential_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
