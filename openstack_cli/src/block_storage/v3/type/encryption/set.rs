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

//! Set Encryption command
//!
//! Wraps invoking of the `v3/types/{type_id}/encryption/{id}` with `PUT` method

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
use crate::common::parse_key_val;
use clap::ValueEnum;
use openstack_sdk::api::block_storage::v3::r#type::encryption::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Update encryption specs for a given volume type.
///
#[derive(Args)]
pub struct EncryptionCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    encryption: Encryption,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// type_id parameter for /v3/types/{type_id}/encryption/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_type_id",
        value_name = "TYPE_ID"
    )]
    type_id: String,

    /// id parameter for /v3/types/{type_id}/encryption/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum ControlLocation {
    BackEnd,
    FrontEnd,
}

/// Encryption Body data
#[derive(Args)]
struct Encryption {
    #[arg(help_heading = "Body parameters", long)]
    cipher: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    control_location: Option<ControlLocation>,

    #[arg(help_heading = "Body parameters", long)]
    key_size: Option<Option<i32>>,

    #[arg(help_heading = "Body parameters", long)]
    provider: Option<String>,
}

/// Encryption response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The encryption algorithm or mode. For example, aes-xts-plain64. The
    /// default value is None.
    ///
    #[serde()]
    #[structable(optional)]
    cipher: Option<String>,

    /// Notional service where encryption is performed. Valid values are
    /// “front-end” or “back-end”. The default value is “front-end”.
    ///
    #[serde()]
    #[structable(optional)]
    control_location: Option<String>,

    /// The date and time when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The resource is deleted or not.
    ///
    #[serde()]
    #[structable(optional)]
    deleted: Option<bool>,

    /// The date and time when the resource was deleted.
    ///
    #[serde()]
    #[structable(optional)]
    deleted_at: Option<String>,

    /// The UUID of the encryption.
    ///
    #[serde()]
    #[structable(optional)]
    encryption_id: Option<String>,

    /// Size of encryption key, in bits. This is usually 256. The default value
    /// is None.
    ///
    #[serde()]
    #[structable(optional)]
    key_size: Option<i32>,

    /// The class that provides encryption support.
    ///
    #[serde()]
    #[structable(optional)]
    provider: Option<String>,

    /// The date and time when the resource was updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl EncryptionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Encryption");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.type_id(&self.path.type_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.encryption data
        let args = &self.encryption;
        let mut encryption_builder = set::EncryptionBuilder::default();
        if let Some(val) = &args.key_size {
            encryption_builder.key_size(*val);
        }

        if let Some(val) = &args.provider {
            encryption_builder.provider(val);
        }

        if let Some(val) = &args.control_location {
            let tmp = match val {
                ControlLocation::BackEnd => set::ControlLocation::BackEnd,
                ControlLocation::FrontEnd => set::ControlLocation::FrontEnd,
            };
            encryption_builder.control_location(tmp);
        }

        if let Some(val) = &args.cipher {
            encryption_builder.cipher(Some(val.into()));
        }

        ep_builder.encryption(encryption_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
