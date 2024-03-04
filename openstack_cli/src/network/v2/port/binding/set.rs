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

//! Set Binding command
//!
//! Wraps invoking of the `v2.0/ports/{port_id}/bindings/{id}` with `PUT` method

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
use openstack_sdk::api::network::v2::port::binding::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct BindingCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    binding: Binding,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    ///
    #[arg(id = "path_param_port_id", value_name = "PORT_ID")]
    port_id: String,

    /// id parameter for /v2.0/ports/{port_id}/bindings/{id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum VnicType {
    AcceleratorDirect,
    AcceleratorDirectPhysical,
    Baremetal,
    Direct,
    DirectPhysical,
    Macvtap,
    Normal,
    RemoteManaged,
    SmartNic,
    Vdpa,
    VirtioForwarder,
}

/// Binding Body data
#[derive(Args)]
struct Binding {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    vnic_type: Option<VnicType>,

    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    profile: Option<Vec<(String, Value)>>,
}

/// Binding response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    host: Option<String>,

    #[serde()]
    #[structable(optional)]
    vif_type: Option<String>,

    #[serde()]
    #[structable(optional)]
    vif_details: Option<String>,

    #[serde()]
    #[structable(optional)]
    vnic_type: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    profile: Option<Value>,

    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,
}

impl BindingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Binding");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.port_id(&self.path.port_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.binding data
        let args = &self.binding;
        let mut binding_builder = set::BindingBuilder::default();
        if let Some(val) = &args.host {
            binding_builder.host(val);
        }

        if let Some(val) = &args.vnic_type {
            let tmp = match val {
                VnicType::AcceleratorDirect => set::VnicType::AcceleratorDirect,
                VnicType::AcceleratorDirectPhysical => set::VnicType::AcceleratorDirectPhysical,
                VnicType::Baremetal => set::VnicType::Baremetal,
                VnicType::Direct => set::VnicType::Direct,
                VnicType::DirectPhysical => set::VnicType::DirectPhysical,
                VnicType::Macvtap => set::VnicType::Macvtap,
                VnicType::Normal => set::VnicType::Normal,
                VnicType::RemoteManaged => set::VnicType::RemoteManaged,
                VnicType::SmartNic => set::VnicType::SmartNic,
                VnicType::Vdpa => set::VnicType::Vdpa,
                VnicType::VirtioForwarder => set::VnicType::VirtioForwarder,
            };
            binding_builder.vnic_type(tmp);
        }

        if let Some(val) = &args.profile {
            binding_builder.profile(val.iter().cloned());
        }

        ep_builder.binding(binding_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
