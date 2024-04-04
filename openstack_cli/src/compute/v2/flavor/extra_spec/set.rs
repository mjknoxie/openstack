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

//! Set ExtraSpec command
//!
//! Wraps invoking of the `v2.1/flavors/{flavor_id}/os-extra_specs/{id}` with `PUT` method

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

use crate::common::parse_key_val;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::flavor::extra_spec::set;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Updates an extra spec, by key, for a flavor, by ID.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Update An Extra Spec For A Flavor")]
pub struct ExtraSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, String>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, String)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_flavor_id",
        value_name = "FLAVOR_ID"
    )]
    flavor_id: String,

    /// id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// ExtraSpec response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ExtraSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ExtraSpec");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.flavor_id(&self.path.flavor_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
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
