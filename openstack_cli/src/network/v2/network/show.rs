//! Shows details for a network.
//!
//! Use the `fields` query parameter to control which fields are
//! returned in the response body. For information, see [Filtering and
//! Column Selection](http://specs.openstack.org/openstack/neutron-
//! specs/specs/api/networking_general_api_information.html#filtering-and-
//! column-selection).
//!
//! Normal response codes: 200
//!
//! Error response codes: 401, 404
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::BoolString;
use crate::common::IntString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::network::find;
use openstack_sdk::api::network::v2::network::get;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct NetworkArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// network_id parameter for /v2.0/networks/{network_id} API
    #[arg()]
    id: String,
}

/// Network show command
pub struct NetworkCmd {
    pub args: NetworkArgs,
}
/// Network response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the network.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the network.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The associated subnets.
    #[serde()]
    #[structable(optional, wide)]
    subnets: Option<VecString>,

    /// The administrative state of the network, which is
    /// up (`true`) or down (`false`).
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The network status. Values are `ACTIVE`, `DOWN`, `BUILD` or `ERROR`.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Indicates whether this network is shared across all tenants. By
    /// default,
    /// only administrative users can change this value.
    #[serde()]
    #[structable(optional, wide)]
    shared: Option<BoolString>,

    /// The ID of the IPv4 address scope that the network is associated with.
    #[serde()]
    #[structable(optional, wide)]
    ipv4_address_scope: Option<String>,

    /// The ID of the IPv6 address scope that the network is associated with.
    #[serde()]
    #[structable(optional, wide)]
    ipv6_address_scope: Option<String>,

    /// Defines whether the network may be used for creation of floating IPs.
    /// Only
    /// networks with this flag may be an external gateway for routers.
    /// The network must have an external routing facility that is not managed
    /// by
    /// the networking service. If the network is updated from external to
    /// internal
    /// the unused floating IPs of this network are automatically deleted when
    /// extension `floatingip-autodelete-internal` is present.
    #[serde(rename = "router:external")]
    #[structable(optional, title = "router:external", wide)]
    router_external: Option<BoolString>,

    /// Indicates whether L2 connectivity is available throughout
    /// the `network`.
    #[serde()]
    #[structable(optional, wide)]
    l2_adjacency: Option<String>,

    /// A list of provider `segment` objects.
    #[serde()]
    #[structable(optional, wide)]
    segments: Option<VecResponseSegments>,

    /// The maximum transmission unit (MTU) value to
    /// address fragmentation. Minimum value is 68 for IPv4, and 1280 for
    /// IPv6.
    #[serde()]
    #[structable(optional, wide)]
    mtu: Option<i32>,

    /// The availability zone for the network.
    #[serde()]
    #[structable(optional, wide)]
    availability_zones: Option<VecString>,

    /// The availability zone candidate for the network.
    #[serde()]
    #[structable(optional, wide)]
    availability_zone_hints: Option<VecString>,

    /// The port security status of the network. Valid values are
    /// enabled (`true`) and disabled (`false`).
    /// This value is used as the default value of `port\_security\_enabled`
    /// field of a newly created port.
    #[serde()]
    #[structable(optional, wide)]
    port_security_enabled: Option<BoolString>,

    #[serde(rename = "provider:network_type")]
    #[structable(optional, title = "provider:network_type", wide)]
    provider_network_type: Option<String>,

    #[serde(rename = "provider:physical_network")]
    #[structable(optional, title = "provider:physical_network", wide)]
    provider_physical_network: Option<String>,

    #[serde(rename = "provider:segmentation_id")]
    #[structable(optional, title = "provider:segmentation_id", wide)]
    provider_segmentation_id: Option<IntString>,

    /// The ID of the QoS policy associated with the network.
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The network is default pool or not.
    #[serde()]
    #[structable(optional, wide)]
    is_default: Option<BoolString>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseSegments {
    provider_segmentation_id: Option<i32>,
    provider_physical_network: Option<String>,
    provider_network_type: Option<String>,
}

impl fmt::Display for ResponseSegments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "provider_segmentation_id={}",
                self.provider_segmentation_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "provider_physical_network={}",
                self.provider_physical_network
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "provider_network_type={}",
                self.provider_network_type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        return write!(f, "{}", data.join(";"));
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseSegments(Vec<ResponseSegments>);
impl fmt::Display for VecResponseSegments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

#[async_trait]
impl Command for NetworkCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Network with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = find::Request::builder();
        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
