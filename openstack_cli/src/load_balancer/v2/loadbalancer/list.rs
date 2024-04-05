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

//! List Loadbalancers command
//!
//! Wraps invoking of the `v2/lbaas/loadbalancers` with `GET` method

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

use openstack_sdk::api::load_balancer::v2::loadbalancer::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists all load balancers for the project.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and column selection](#filtering).
///
/// Administrative users can specify a project ID that is different than their
/// own to list load balancers for other projects.
///
/// The list might be empty.
///
#[derive(Args)]
#[command(about = "List Load Balancers")]
pub struct LoadbalancersCommand {
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
struct PathParameters {}
/// Loadbalancers response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of JSON objects defining “additional VIPs”. The format for these
    /// is `{"subnet_id": <subnet_id>, "ip_address": <ip_address>}`, where the
    /// `subnet_id` field is mandatory and the `ip_address` field is optional.
    /// Additional VIP subnets must all belong to the same network as the
    /// primary VIP.
    ///
    /// **New in version 2.26**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    additional_vips: Option<Value>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<bool>,

    /// An availability zone name.
    ///
    #[serde()]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional, wide)]
    flavor_id: Option<String>,

    /// The ID of the load balancer.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The associated listener IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    listeners: Option<Value>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional, wide)]
    operating_status: Option<String>,

    /// The associated pool IDs, if any.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    pools: Option<Value>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// Provider name for the load balancer.
    ///
    #[serde()]
    #[structable(optional, wide)]
    provider: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional, wide)]
    provisioning_status: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_address: Option<String>,

    /// The ID of the network for the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_network_id: Option<String>,

    /// The ID of the Virtual IP (VIP) port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_port_id: Option<String>,

    /// The ID of the QoS Policy which will apply to the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_qos_policy_id: Option<String>,

    /// The ID of the subnet for the Virtual IP (VIP).
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_subnet_id: Option<String>,

    /// The VIP vNIC type used for the load balancer. One of `normal` or
    /// `direct`.
    ///
    /// **New in version 2.28**
    ///
    #[serde()]
    #[structable(optional, wide)]
    vip_vnic_type: Option<String>,
}

impl LoadbalancersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Loadbalancers");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = list::Request::builder();

        // Set path parameters
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
