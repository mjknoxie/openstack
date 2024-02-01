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
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_key_val;
use openstack_sdk::api::compute::v2::server::create_backup_21;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

#[derive(Args, Clone, Debug)]
#[command(about = "Create Server Back Up (createBackup Action) (microversion = 2.1)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    create_backup: CreateBackup,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// CreateBackup Body data
#[derive(Args, Debug, Clone)]
struct CreateBackup {
    /// The name of the image to be backed up.
    #[arg(long)]
    name: String,

    /// The type of the backup, for example, `daily`.
    #[arg(long)]
    backup_type: String,

    /// The rotation of the back up image, the oldest image will be removed
    /// when image count
    /// exceed the rotation count.
    #[arg(long)]
    rotation: String,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is
    /// 255 bytes each.
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,
}

/// Server action command
pub struct ServerCmd {
    /// Command arguments
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The UUID for the resulting image snapshot.
    #[serde()]
    #[structable()]
    image_id: String,
}

#[async_trait]
impl OSCCommand for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_backup_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.create_backup data
        let args = &self.args.create_backup;
        let mut create_backup_builder = create_backup_21::CreateBackupBuilder::default();

        create_backup_builder.name(args.name.clone());

        create_backup_builder.backup_type(args.backup_type.clone());

        create_backup_builder.rotation(args.rotation.clone());

        if let Some(val) = &args.metadata {
            create_backup_builder.metadata(val.iter().cloned());
        }

        ep_builder.create_backup(create_backup_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
