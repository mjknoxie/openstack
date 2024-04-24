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

//! Create Group command [microversion = 3.13]
//!
//! Wraps invoking of the `v3/groups` with `POST` method

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

use openstack_sdk::api::block_storage::v3::group::create_313;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Create a new group.
///
#[derive(Args)]
pub struct GroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A group object.
    ///
    #[command(flatten)]
    group: Group,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Group Body data
#[derive(Args, Clone)]
struct Group {
    /// The name of the availability zone.
    ///
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    /// The group description.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The group type ID.
    ///
    #[arg(help_heading = "Body parameters", long)]
    group_type: String,

    /// The group name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The list of volume types. In an environment with multiple-storage back
    /// ends, the scheduler determines where to send the volume based on the
    /// volume type. For information about how to use volume types to create
    /// multiple- storage back ends, see
    /// [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    volume_types: Vec<String>,
}

/// Group response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The date and time when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The group description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the group snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    group_snapshot_id: Option<String>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable(optional)]
    group_type: Option<String>,

    /// The UUID of the group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The group name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The UUID of the volume group project.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The group replication status.
    ///
    #[serde()]
    #[structable(optional)]
    replication_status: Option<String>,

    /// The UUID of the source group.
    ///
    #[serde()]
    #[structable(optional)]
    source_group_id: Option<String>,

    /// The status of the generic group.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The list of volume types. In an environment with multiple-storage back
    /// ends, the scheduler determines where to send the volume based on the
    /// volume type.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    volume_types: Option<Value>,

    /// A list of volume ids, available only when list_volume set true.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    volumes: Option<Value>,
}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Group");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_313::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.13");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.group data
        let args = &self.group;
        let mut group_builder = create_313::GroupBuilder::default();
        if let Some(val) = &args.description {
            group_builder.description(Some(val.into()));
        }

        group_builder.group_type(&args.group_type);

        if let Some(val) = &args.name {
            group_builder.name(Some(val.into()));
        }

        group_builder.volume_types(
            args.volume_types
                .iter()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
        );

        if let Some(val) = &args.availability_zone {
            group_builder.availability_zone(Some(val.into()));
        }

        ep_builder.group(group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}