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

//! Delete Property command
//!
//! Wraps invoking of the `v2/metadefs/namespaces/{namespace_name}/properties/{property_name}` with `DELETE` method

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
use openstack_sdk::api::image::v2::metadef::namespace::property::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Removes a property definition from a namespace.
///
/// When you successfully delete a property from a namespace, the response is
/// empty and the response code is `204`.
///
/// Normal response codes: 204
///
/// Error response codes: 401, 403, 404
///
#[derive(Args)]
#[command(about = "Remove property definition")]
pub struct PropertyCommand {
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
    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_namespace_name",
        value_name = "NAMESPACE_NAME"
    )]
    namespace_name: String,

    /// property_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_property_name",
        value_name = "PROPERTY_NAME"
    )]
    property_name: String,
}
/// Property response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl PropertyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Property");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.namespace_name(&self.path.namespace_name);
        ep_builder.property_name(&self.path.property_name);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
