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

//! Show Subnet command
//!
//! Wraps invoking of the `v2.0/subnets/{subnet_id}` with `GET` method

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

use crate::common::BoolString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::subnet::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows details for a subnet.
///
/// Use the fields query parameter to filter the results.
///
/// Normal response codes: 200
///
/// Error response codes: 401, 404
///
#[derive(Args)]
#[command(about = "Show subnet details")]
pub struct SubnetCommand {
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
    /// subnet_id parameter for /v2.0/subnets/{subnet_id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Subnet response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    ///
    #[serde()]
    #[structable(optional)]
    ip_version: Option<i32>,

    /// The ID of the network to which the subnet belongs.
    ///
    #[serde()]
    #[structable(optional)]
    network_id: Option<String>,

    /// The ID of the subnet pool associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    subnetpool_id: Option<String>,

    /// The CIDR of the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    ///
    #[serde()]
    #[structable(optional)]
    allocation_pools: Option<Value>,

    /// List of dns name servers associated with the subnet.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    dns_nameservers: Option<Value>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    ///
    #[serde()]
    #[structable(optional)]
    host_routes: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled for the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    enable_dhcp: Option<BoolString>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde()]
    #[structable(optional)]
    ipv6_ra_mode: Option<String>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde()]
    #[structable(optional)]
    ipv6_address_mode: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The service types associated with the subnet.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    service_types: Option<Value>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Whether to publish DNS records for IPs from this subnet.
    ///
    #[serde()]
    #[structable(optional)]
    dns_publish_fixed_ip: Option<BoolString>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    segment_id: Option<String>,
}

impl SubnetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Subnet");

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
