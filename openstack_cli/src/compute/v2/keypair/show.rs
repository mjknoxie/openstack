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

//! Show Keypair command
//!
//! Wraps invoking of the `v2.1/os-keypairs/{id}` with `GET` method

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

use openstack_sdk::api::compute::v2::keypair::find;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows details for a keypair that is associated with the account.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "Show Keypair Details")]
pub struct KeypairCommand {
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
    #[arg(long)]
    user_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-keypairs/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Keypair response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The user\_id for a keypair.
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// A boolean indicates whether this keypair is deleted or not.
    /// The value is always `false` (not deleted).
    #[serde()]
    #[structable(optional)]
    deleted: Option<bool>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// It is always `null`.
    #[serde()]
    #[structable(optional)]
    deleted_at: Option<String>,

    /// It is always `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The keypair ID.
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// The name for the keypair.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The keypair public key.
    #[serde()]
    #[structable(optional)]
    public_key: Option<String>,

    /// The fingerprint for the keypair.
    #[serde()]
    #[structable(optional)]
    fingerprint: Option<String>,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,
}

impl KeypairCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Keypair");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
