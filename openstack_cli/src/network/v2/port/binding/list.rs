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

//! List Bindings command
//!
//! Wraps invoking of the `v2.0/ports/{port_id}/bindings` with `GET` method

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

use openstack_sdk::api::network::v2::port::binding::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Normal response codes: 200
///
/// Error response codes: 401, 404
///
#[derive(Args)]
#[command(about = "Show Port Binding of a Port")]
pub struct BindingsCommand {
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
    /// host query parameter for /v2.0/ports/{port_id}/bindings API
    ///
    #[arg(long)]
    host: Option<String>,

    /// vif_type query parameter for /v2.0/ports/{port_id}/bindings API
    ///
    #[arg(long)]
    vif_type: Option<String>,

    /// vnic_type query parameter for /v2.0/ports/{port_id}/bindings API
    ///
    #[arg(long, value_parser = ["accelerator-direct","accelerator-direct-physical","baremetal","direct","direct-physical","macvtap","normal","remote-managed","smart-nic","vdpa","virtio-forwarder"])]
    vnic_type: Option<String>,

    /// status query parameter for /v2.0/ports/{port_id}/bindings API
    ///
    #[arg(long)]
    status: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    ///
    #[arg(id = "path_param_port_id", value_name = "PORT_ID")]
    port_id: String,
}
/// Bindings response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The hostname of the system the agent is running on.
    ///
    #[serde()]
    #[structable(optional)]
    host: Option<String>,

    /// The type of which mechanism is used for the port. An API consumer like
    /// nova can use this to determine an appropriate way to attach a device
    /// (for example an interface of a virtual server) to the port. Available
    /// values currently defined includes `ovs`, `bridge`, `macvtap`, `hw_veb`,
    /// `hostdev_physical`, `vhostuser`, `distributed` and `other`. There are
    /// also special values: `unbound` and `binding_failed`. `unbound` means
    /// the port is not bound to a networking back-end. `binding_failed` means
    /// an error that the port failed to be bound to a networking back-end.
    ///
    #[serde()]
    #[structable(optional)]
    vif_type: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port_filter` and
    /// `ovs_hybrid_plug`. `port_filter` is a boolean indicating the networking
    /// service provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing. `ovs_hybrid_plug` is a boolean used to inform an
    /// API consumer like nova that the hybrid plugging strategy for OVS should
    /// be used.
    ///
    #[serde()]
    #[structable(optional)]
    vif_details: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments.
    ///
    #[serde()]
    #[structable(optional)]
    vnic_type: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. The networking API does not define a specific format of this
    /// field. If the update request is null this response field will be {}.
    ///
    #[serde()]
    #[structable(optional)]
    profile: Option<HashMapStringValue>,

    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,
}
/// HashMap of `Value` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl BindingsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Bindings");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.port_id(&self.path.port_id);
        // Set query parameters
        if let Some(val) = &self.query.host {
            ep_builder.host(val);
        }
        if let Some(val) = &self.query.vif_type {
            ep_builder.vif_type(val);
        }
        if let Some(val) = &self.query.vnic_type {
            ep_builder.vnic_type(val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
