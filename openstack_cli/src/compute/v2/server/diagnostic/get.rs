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

//! Get Diagnostic command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/diagnostics` with `GET` method

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

use openstack_sdk::api::compute::v2::server::diagnostic::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows basic usage data for a server.
///
/// Policy defaults enable only users with the administrative role. Cloud
/// providers can change these permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), notfound(404),
/// conflict(409), notimplemented(501)
///
#[derive(Args)]
#[command(about = "Show Server Diagnostics")]
pub struct DiagnosticCommand {
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
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    ///
    #[arg(id = "path_param_server_id", value_name = "SERVER_ID")]
    server_id: String,
}
/// Diagnostic response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The list of dictionaries with detailed information about VM CPUs.
    /// Following fields are presented in each dictionary:
    ///
    /// - `id` - the ID of CPU (Integer)
    /// - `time` - CPU Time in nano seconds (Integer)
    /// - `utilisation` - CPU utilisation in percents (Integer)
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    cpu_details: Option<Value>,

    /// The list of dictionaries with detailed information about VM disks.
    /// Following fields are presented in each dictionary:
    ///
    /// - `read_bytes` - Disk reads in bytes (Integer)
    /// - `read_requests` - Read requests (Integer)
    /// - `write_bytes` - Disk writes in bytes (Integer)
    /// - `write_requests` - Write requests (Integer)
    /// - `errors_count` - Disk errors (Integer)
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    disk_details: Option<Value>,

    /// The driver on which the VM is running. Possible values are:
    ///
    /// - `libvirt`
    /// - `xenapi`
    /// - `hyperv`
    /// - `vmwareapi`
    /// - `ironic`
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    driver: Option<String>,

    /// Indicates whether or not a config drive was used for this server.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    config_drive: Option<bool>,

    /// The hypervisor on which the VM is running. Examples for libvirt driver
    /// may be: `qemu`, `kvm` or `xen`.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    hypervisor: Option<String>,

    /// The hypervisor OS.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    hypervisor_os: Option<String>,

    /// Id of the resource
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The dictionary with information about VM memory usage. Following fields
    /// are presented in the dictionary:
    ///
    /// - `maximum` - Amount of memory provisioned for the VM in MiB (Integer)
    /// - `used` - Amount of memory that is currently used by the guest
    ///   operating system and its applications in MiB (Integer)
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    memory_details: Option<Value>,

    /// Name
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The list of dictionaries with detailed information about VM NICs.
    /// Following fields are presented in each dictionary:
    ///
    /// - `mac_address` - Mac address of the interface (String)
    /// - `rx_octets` - Received octets (Integer)
    /// - `rx_errors` - Received errors (Integer)
    /// - `rx_drop` - Received packets dropped (Integer)
    /// - `rx_packets` - Received packets (Integer)
    /// - `rx_rate` - Receive rate in bytes (Integer)
    /// - `tx_octets` - Transmitted Octets (Integer)
    /// - `tx_errors` - Transmit errors (Integer)
    /// - `tx_drop` - Transmit dropped packets (Integer)
    /// - `tx_packets` - Transmit packets (Integer)
    /// - `tx_rate` - Transmit rate in bytes (Integer)
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    nic_details: Option<Value>,

    /// The number of vCPUs.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    num_cpus: Option<i32>,

    /// The number of disks.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    num_disks: Option<i32>,

    /// The number of vNICs.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    num_nics: Option<i32>,

    /// A string enum denoting the current state of the VM. Possible values
    /// are:
    ///
    /// - `pending`
    /// - `running`
    /// - `paused`
    /// - `shutdown`
    /// - `crashed`
    /// - `suspended`
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The amount of time in seconds that the VM has been running.
    ///
    /// **New in version 2.48**
    ///
    #[serde()]
    #[structable(optional)]
    uptime: Option<i32>,
}

impl DiagnosticCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Diagnostic");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
