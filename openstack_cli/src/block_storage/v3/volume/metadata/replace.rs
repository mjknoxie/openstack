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

//! Set Metadata command
//!
//! Wraps invoking of the `v3/volumes/{volume_id}/metadata` with `PUT` method

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
use openstack_sdk::api::block_storage::v3::volume::metadata::replace;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct MetadataCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Vec<(String, String)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// volume_id parameter for /v3/volumes/{volume_id}/encryption/{id} API
    #[arg(id = "path_param_volume_id", value_name = "VOLUME_ID")]
    volume_id: String,
}
/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
struct ResponseData(HashMap<String, String>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(
            self.0
                .iter()
                .map(|(k, v)| Vec::from([k.clone(), v.clone()])),
        );
        (headers, rows)
    }
}

impl MetadataCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Metadata");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = replace::Request::builder();

        // Set path parameters
        ep_builder.volume_id(&self.path.volume_id);
        // Set query parameters
        // Set body parameters
        // Set Request.metadata data
        let args = &self.metadata;

        ep_builder.metadata(args.iter().cloned());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
