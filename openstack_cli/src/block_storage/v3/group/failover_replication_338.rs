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

//! Action Group command [microversion = 3.38]
//!
//! Wraps invoking of the `v3/groups/action` with `POST` method

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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::group::failover_replication_338;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct GroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    failover_replication: FailoverReplication,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// FailoverReplication Body data
#[derive(Args, Clone)]
struct FailoverReplication {
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    allow_attached_volume: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    secondary_backend_id: Option<String>,
}

/// Group response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Group");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = failover_replication_338::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.38");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.failover_replication data
        let args = &self.failover_replication;
        let mut failover_replication_builder =
            failover_replication_338::FailoverReplicationBuilder::default();
        if let Some(val) = &args.allow_attached_volume {
            failover_replication_builder.allow_attached_volume(*val);
        }

        if let Some(val) = &args.secondary_backend_id {
            failover_replication_builder.secondary_backend_id(Some(val.into()));
        }

        ep_builder.failover_replication(failover_replication_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
