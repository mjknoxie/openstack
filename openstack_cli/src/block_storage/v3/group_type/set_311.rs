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

//! Set GroupType command [microversion = 3.11]
//!
//! Wraps invoking of the `v3/group_types/{id}` with `PUT` method

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

use openstack_sdk::api::block_storage::v3::group_type::set_311;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct GroupTypeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    group_type: GroupType,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/group_types/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// GroupType Body data
#[derive(Args, Clone)]
struct GroupType {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    is_public: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// GroupType response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The group type description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// A set of key and value pairs that contains the specifications for a
    /// group type.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    group_specs: Option<Value>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Whether the group type is publicly visible.
    ///
    #[serde()]
    #[structable(optional)]
    is_public: Option<bool>,

    /// The group type name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl GroupTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set GroupType");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_311::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.11");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.group_type data
        let args = &self.group_type;
        let mut group_type_builder = set_311::GroupTypeBuilder::default();
        if let Some(val) = &args.name {
            group_type_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            group_type_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.is_public {
            group_type_builder.is_public(*val);
        }

        ep_builder.group_type(group_type_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}