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

//! Show Message command
//!
//! Wraps invoking of the `v3/messages/{id}` with `GET` method

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

use openstack_sdk::api::block_storage::v3::message::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Return the given message.
///
#[derive(Args)]
pub struct MessageCommand {
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
    /// id parameter for /v3/messages/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Message response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    #[serde()]
    #[structable()]
    created_at: String,

    /// The id of the event to this message, this id could eventually be
    /// translated into `user_message`.
    ///
    #[serde()]
    #[structable()]
    event_id: String,

    /// The expire time of the message, this message could be deleted after
    /// this time.
    ///
    #[serde()]
    #[structable(optional)]
    guaranteed_until: Option<String>,

    /// The UUID for the message.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// Links for the message.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// The level of the message, possible value is only ‘ERROR’ now.
    ///
    #[serde()]
    #[structable()]
    message_level: String,

    /// The id of the request during which the message was created.
    ///
    #[serde()]
    #[structable()]
    request_id: String,

    /// The resource type corresponding to `resource_uuid`.
    ///
    #[serde()]
    #[structable(optional)]
    resource_type: Option<String>,

    /// The UUID of the resource during whose operation the message was
    /// created.
    ///
    #[serde()]
    #[structable(optional)]
    resource_uuid: Option<String>,

    /// The translated readable message corresponding to `event_id`.
    ///
    #[serde()]
    #[structable()]
    user_message: String,
}

impl MessageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Message");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
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