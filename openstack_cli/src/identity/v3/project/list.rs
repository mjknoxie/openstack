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

//! List Projects command
//!
//! Wraps invoking of the `v3/projects` with `GET` method

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

use openstack_sdk::api::identity::v3::project::list;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Lists projects.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/projects`
///
#[derive(Args)]
#[command(about = "List projects")]
pub struct ProjectsCommand {
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
    /// Filters the response by a domain ID.
    ///
    #[arg(long)]
    domain_id: Option<String>,

    /// If set to true, then only enabled projects will be returned. Any value
    /// other than 0 (including no value) will be interpreted as true.
    ///
    #[arg(long)]
    enabled: Option<bool>,

    /// If this is specified as true, then only projects acting as a domain are
    /// included. Otherwise, only projects that are not acting as a domain are
    /// included.
    ///
    #[arg(long)]
    is_domain: Option<bool>,

    /// Filters the response by a resource name.
    ///
    #[arg(long)]
    name: Option<String>,

    /// Filters the response by a parent ID.
    ///
    #[arg(long)]
    parent_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Projects response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID for the project.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    is_domain: Option<bool>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    ///
    #[serde()]
    #[structable(optional, wide)]
    parent_id: Option<String>,

    /// The name of the project.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    options: Option<ResponseOptions>,
}
/// Vector of `String` response type
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
/// `struct` response type
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

impl ProjectsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Projects");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.domain_id {
            ep_builder.domain_id(val.clone());
        }
        if let Some(val) = &self.query.enabled {
            ep_builder.enabled(*val);
        }
        if let Some(val) = &self.query.is_domain {
            ep_builder.is_domain(*val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.query.parent_id {
            ep_builder.parent_id(val.clone());
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
