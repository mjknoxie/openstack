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

//! List PortForwardings command
//!
//! Wraps invoking of the `v2.0/floatingips/{floatingip_id}/port_forwardings` with `GET` method

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

use openstack_sdk::api::network::v2::floatingip::port_forwarding::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists floating IP port forwardings that the project has access to.
///
/// Default policy settings return only the port forwardings associated to
/// floating IPs owned by the project of the user submitting the request,
/// unless the user has administrative role.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
///
/// Normal response codes: 200
///
/// Error response codes: 400, 404
///
#[derive(Args)]
#[command(about = "List floating IP port forwardings")]
pub struct PortForwardingsCommand {
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
    /// id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long)]
    id: Option<String>,

    /// external_port query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long)]
    external_port: Option<f32>,

    /// protocol query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long, value_parser = ["dccp","icmp","ipv6-icmp","sctp","tcp","udp"])]
    protocol: Option<String>,

    /// internal_port_id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long)]
    internal_port_id: Option<String>,

    /// description query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long)]
    description: Option<String>,

    /// external_port_range query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(long)]
    external_port_range: Option<f32>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    ///
    #[arg(id = "path_param_floatingip_id", value_name = "FLOATINGIP_ID")]
    floatingip_id: String,
}
/// PortForwardings response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_port: Option<f32>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port: Option<f32>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_ip_address: Option<String>,

    /// The IP protocol used in the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    protocol: Option<String>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port_id: Option<String>,

    /// A text describing the rule, which helps users to manage/find easily
    /// theirs rules.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port_range: Option<f32>,
}

impl PortForwardingsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List PortForwardings");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.floatingip_id(&self.path.floatingip_id);
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &self.query.external_port {
            ep_builder.external_port(*val);
        }
        if let Some(val) = &self.query.protocol {
            ep_builder.protocol(val.clone());
        }
        if let Some(val) = &self.query.internal_port_id {
            ep_builder.internal_port_id(val.clone());
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &self.query.external_port_range {
            ep_builder.external_port_range(*val);
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
