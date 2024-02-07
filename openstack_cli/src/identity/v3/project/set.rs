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

//! Set Project command
//!
//! Wraps invoking of the `v3/projects/{project_id}` with `PATCH` method

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
use openstack_sdk::api::identity::v3::project::find;
use openstack_sdk::api::identity::v3::project::set;
use openstack_sdk::api::QueryAsync;

use std::fmt;
use structable_derive::StructTable;

/// Updates a project.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/project`
#[derive(Args)]
#[command(about = "Update project")]
pub struct ProjectCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    project: Project,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// Options Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Options {
    #[arg(action=clap::ArgAction::Set, long)]
    immutable: Option<bool>,
}

/// Project Body data
#[derive(Args)]
struct Project {
    /// The description of the project.
    #[arg(long)]
    description: Option<String>,

    /// The ID of the new domain for the project. The ability to change the
    /// domain
    /// of a project is now deprecated, and will be removed in subequent
    /// release.
    /// It is already disabled by default in most Identity service
    /// implementations.
    #[arg(long)]
    domain_id: Option<String>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled.
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled.
    #[arg(action=clap::ArgAction::Set, long)]
    is_domain: Option<bool>,

    #[arg(long)]
    parent_id: Option<String>,

    /// The name of the project, which must be unique within the
    /// owning domain. A project can have the same name as its domain.
    #[arg(long)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    /// Tags can be used to classify projects into groups.
    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[command(flatten)]
    options: Option<Options>,
}

/// Project response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID for the project.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the project.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain for the project.
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The ID of the parent for the project.
    ///
    ///
    /// **New in version 3.4**
    #[serde()]
    #[structable(optional)]
    parent_id: Option<String>,

    /// The name of the project.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
}
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    immutable: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "immutable={}",
            self.immutable
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

impl ProjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Project");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.project data
        let args = &self.project;
        let mut project_builder = set::ProjectBuilder::default();
        if let Some(val) = &args.description {
            project_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            project_builder.domain_id(Some(val.into()));
        }

        if let Some(val) = &args.enabled {
            project_builder.enabled(*val);
        }

        if let Some(val) = &args.is_domain {
            project_builder.is_domain(*val);
        }

        if let Some(val) = &args.parent_id {
            project_builder.parent_id(Some(val.into()));
        }

        if let Some(val) = &args.name {
            project_builder.name(val.clone());
        }

        if let Some(val) = &args.tags {
            project_builder.tags(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &val.immutable {
                options_builder.immutable(*val);
            }
            project_builder.options(options_builder.build().expect("A valid object"));
        }

        ep_builder.project(project_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
