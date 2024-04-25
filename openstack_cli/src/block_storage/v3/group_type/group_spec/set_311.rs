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

//! Set GroupSpec command [microversion = 3.11]
//!
//! Wraps invoking of the `v3/group_types/{group_type_id}/group_specs/{id}` with `PUT` method

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

use crate::common::parse_key_val_opt;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::group_type::group_spec::set_311;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct GroupSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val_opt::<String, String>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, Option<String>)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// group_type_id parameter for
    /// /v3/group_types/{group_type_id}/group_specs/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group_type_id",
        value_name = "GROUP_TYPE_ID"
    )]
    group_type_id: String,

    /// id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// GroupSpec response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl GroupSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set GroupSpec");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_311::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.11");

        // Set path parameters
        ep_builder.group_type_id(&self.path.group_type_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.properties {
            ep_builder.properties(
                properties
                    .iter()
                    .map(|(k, v)| (k, v.as_ref().map(Into::into))),
            );
        }

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