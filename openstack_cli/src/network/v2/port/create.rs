//! Creates multiple ports in a single request. Specify a list of ports in the
//! request body.
//!
//! Guarantees the atomic completion of the bulk operation.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 404, 409
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

use crate::common::parse_json;
use crate::common::parse_key_val;
use crate::common::BoolString;
use clap::ValueEnum;
use openstack_sdk::api::network::v2::port::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct PortArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    port: Port,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum NumaAffinityPolicy {
    Preferred,
    Required,
    Legacy,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum BindingVnicType {
    RemoteManaged,
    Macvtap,
    DirectPhysical,
    AcceleratorDirect,
    Baremetal,
    AcceleratorDirectPhysical,
    Direct,
    Normal,
    VirtioForwarder,
    Vdpa,
    SmartNic,
}

/// Port Body data
#[derive(Args, Debug, Clone)]
struct Port {
    /// Human-readable name of the resource.
    /// Default is an empty string.
    #[arg(long)]
    name: Option<String>,

    /// The ID of the attached network.
    #[arg(long)]
    network_id: Option<String>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    /// Default is `true`.
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// The MAC address of the port.
    /// If unspecified, a MAC address is automatically generated.
    #[arg(long)]
    mac_address: Option<String>,

    /// The IP addresses for the port.
    /// If you would like to assign multiple IP addresses for the port,
    /// specify multiple entries in this field.
    /// Each entry consists of IP address (`ip\_address`) and the subnet ID
    /// from which the IP address is assigned (`subnet\_id`).
    ///
    ///
    /// * If you specify both a subnet ID and an IP address, OpenStack
    /// Networking
    /// tries to allocate the IP address on that subnet to the port.
    /// * If you specify only a subnet ID, OpenStack Networking allocates
    /// an available IP from that subnet to the port.
    /// * If you specify only an IP address, OpenStack Networking
    /// tries to allocate the IP address if the address is a valid IP
    /// for any of the subnets on the specified network.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    fixed_ips: Option<Vec<Value>>,

    /// The ID of the device that uses this port.
    /// For example, a server instance or a logical router.
    #[arg(long)]
    device_id: Option<String>,

    /// The entity type that uses this port.
    /// For example, `compute:nova` (server instance), `network:dhcp`
    /// (DHCP agent) or `network:router\_interface` (router interface).
    #[arg(long)]
    device_owner: Option<String>,

    /// The ID of the project that owns the resource.
    /// Only administrative and users with advsvc role can specify
    /// a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[arg(long)]
    tenant_id: Option<String>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair
    /// object contains an `ip\_address` and `mac\_address`. While the
    /// `ip\_address` is required, the `mac\_address` will be taken from the
    /// port if not specified. The value of `ip\_address` can be an IP Address
    /// or a CIDR (if supported by the underlying extension plugin).
    /// A server connected to the port can send a packet with source address
    /// which
    /// matches one of the specified allowed address pairs.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    allowed_address_pairs: Option<Vec<Value>>,

    /// A set of zero or more extra DHCP option pairs. An
    /// option pair consists of an option value and name.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    extra_dhcp_opts: Option<Vec<Value>>,

    #[arg(long)]
    device_profile: Option<String>,

    /// Admin-only. A dict, at the top level keyed by mechanism driver
    /// aliases (as defined in setup.cfg). To following values can be used to
    /// control Open vSwitch’s Userspace Tx packet steering feature:
    ///
    ///
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "hash"}}}`
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "thread"}}}`
    ///
    ///
    /// If omitted the default is defined by Open vSwitch.
    /// The field cannot be longer than 4095 characters.
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    hints: Option<Vec<(String, Value)>>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    #[arg(long)]
    numa_affinity_policy: Option<NumaAffinityPolicy>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port.
    /// The valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic`
    /// and `remote-managed`.
    /// What type of vNIC is actually available depends on deployments.
    /// The default is `normal`.
    #[arg(long)]
    binding_vnic_type: Option<BindingVnicType>,

    /// The ID of the host where the port resides.
    /// The default is an empty string.
    #[arg(long)]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// This field is only meant for machine-machine communication for compute
    /// services like Nova, Ironic or Zun to pass information to a Neutron
    /// back-end. It should not be used by multiple services concurrently or by
    /// cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field.
    /// The default is an empty dictionary.
    /// If you update it with null then it is treated like {} in the response.
    /// Since the port-mac-address-override extension the
    /// `device\_mac\_address`
    /// field of the binding:profile can be used to provide the MAC address of
    /// the
    /// physical device a direct-physical port is being bound to. If provided,
    /// then
    /// the `mac\_address` field of the port resource will be updated to the
    /// MAC
    /// from the active binding.
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    binding_profile: Option<Vec<(String, Value)>>,

    /// The port security status. A valid value is
    /// enabled (`true`) or disabled (`false`).
    /// If port security is enabled for the port,
    /// security group rules and anti-spoofing rules are applied to
    /// the traffic on the port. If disabled, no such rules are applied.
    #[arg(action=clap::ArgAction::Set, long)]
    port_security_enabled: Option<bool>,

    /// QoS policy associated with the port.
    #[arg(long)]
    qos_policy_id: Option<String>,

    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,

    /// The uplink status propagation of the port. Valid values are
    /// enabled (`true`) and disabled (`false`).
    #[arg(action=clap::ArgAction::Set, long)]
    propagate_uplink_status: Option<bool>,

    /// A valid DNS name.
    #[arg(long)]
    dns_name: Option<String>,

    /// A valid DNS domain.
    #[arg(long)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,

    /// The IDs of security groups applied to the port.
    #[arg(action=clap::ArgAction::Append, long)]
    security_groups: Option<Vec<String>>,
}

/// Port create command
pub struct PortCmd {
    pub args: PortArgs,
}
/// Port response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the resource.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[serde()]
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The MAC address of the port. If the port uses the `direct-physical`
    /// `vnic\_type` then the value of this field is overwritten with the MAC
    /// address provided in the active binding:profile if any.
    #[serde()]
    #[structable(optional, wide)]
    mac_address: Option<String>,

    /// The IP addresses for the port. If the port has multiple IP addresses,
    /// this field has multiple entries. Each entry consists of IP address
    /// (`ip\_address`) and the subnet ID from which the IP address
    /// is assigned (`subnet\_id`).
    #[serde()]
    #[structable(optional, wide)]
    fixed_ips: Option<VecResponseFixedIps>,

    /// The ID of the device that uses this port.
    /// For example, a server instance or a logical router.
    #[serde()]
    #[structable(optional, wide)]
    device_id: Option<String>,

    /// The entity type that uses this port.
    /// For example, `compute:nova` (server instance), `network:dhcp`
    /// (DHCP agent) or `network:router\_interface` (router interface).
    #[serde()]
    #[structable(optional, wide)]
    device_owner: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The port status. Values are `ACTIVE`, `DOWN`,
    /// `BUILD` and `ERROR`.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair
    /// object contains an `ip\_address` and `mac\_address`. While the
    /// `ip\_address` is required, the `mac\_address` will be taken from the
    /// port if not specified. The value of `ip\_address` can be an IP Address
    /// or a CIDR (if supported by the underlying extension plugin).
    /// A server connected to the port can send a packet with source address
    /// which
    /// matches one of the specified allowed address pairs.
    #[serde()]
    #[structable(optional, wide)]
    allowed_address_pairs: Option<VecResponseAllowedAddressPairs>,

    /// Status of the underlying data plane of a port.
    #[serde()]
    #[structable(optional, wide)]
    data_plane_status: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An
    /// option pair consists of an option value and name.
    #[serde()]
    #[structable(optional, wide)]
    extra_dhcp_opts: Option<VecHashMapStringValue>,

    /// Indicates when ports use either `deferred`, `immediate` or no IP
    /// allocation (`none`).
    #[serde()]
    #[structable(optional, wide)]
    ip_allocation: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    device_profile: Option<String>,

    /// Admin-only. The following values control Open vSwitch’s Userspace Tx
    /// packet steering feature:
    ///
    ///
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "hash|thread"}}}`
    #[serde()]
    #[structable(optional, wide)]
    hints: Option<HashMapStringValue>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    #[serde()]
    #[structable(optional, wide)]
    numa_affinity_policy: Option<String>,

    /// Expose Placement resources (i.e.: `minimum-bandwidth`) and
    /// traits (i.e.: `vnic-type`, `physnet`) requested by a port to
    /// Nova and Placement. A `resource\_request` object contains
    /// `request\_groups` and `same\_subtree` keys. `request\_groups` is a list
    /// of dicts, where each dict represents one group of resources and traits
    /// that needs to be fulfilled from a single resource provider. Every dict
    /// in
    /// the list must contain `id`, `required` and `resources` keys. The
    /// `id` field is a string which represents a unique UUID that is generated
    /// for each group by combining the `port\_id` and UUIDs of the QoS rules
    /// contributing to the group via the UUID5 method. `required` key contains
    /// the traits (generated from the `vnic\_type` and the `physnet`) required
    /// by the port, and a `resources` key contains a mapping of requested
    /// resource class name and requested amount from the QoS policy.
    /// `same\_subtree` key contains a list of `id` values from every resource
    /// group.
    #[serde()]
    #[structable(optional, wide)]
    resource_request: Option<String>,

    /// The type of which mechanism is used for the port.
    /// An API consumer like nova can use this to determine an appropriate way
    /// to
    /// attach a device (for example an interface of a virtual server) to the
    /// port.
    /// Available values currently defined includes
    /// `ovs`, `bridge`, `macvtap`, `hw\_veb`, `hostdev\_physical`,
    /// `vhostuser`, `distributed` and `other`.
    /// There are also special values: `unbound` and `binding\_failed`.
    /// `unbound` means the port is
    /// not bound to a networking back-end. `binding\_failed` means an error
    /// that the port failed to be bound to a networking back-end.
    #[serde(rename = "binding:vif_type")]
    #[structable(optional, title = "binding:vif_type", wide)]
    binding_vif_type: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port\_filter` and
    /// `ovs\_hybrid\_plug`.
    /// `port\_filter` is a boolean indicating the networking service
    /// provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing.
    /// `ovs\_hybrid\_plug` is a boolean used to inform an API consumer
    /// like nova that the hybrid plugging strategy for OVS should be used.
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, title = "binding:vif_details", wide)]
    binding_vif_details: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port.
    /// The valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic`
    /// and `remote-managed`.
    /// What type of vNIC is actually available depends on deployments.
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, title = "binding:vnic_type", wide)]
    binding_vnic_type: Option<String>,

    /// The ID of the host where the port resides.
    #[serde(rename = "binding:host_id")]
    #[structable(optional, title = "binding:host_id", wide)]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// The networking API does not define a specific format of this field.
    /// If the update request is null this response field will be {}.
    #[serde(rename = "binding:profile")]
    #[structable(optional, title = "binding:profile", wide)]
    binding_profile: Option<HashMapStringValue>,

    /// The port security status. A valid value is
    /// enabled (`true`) or disabled (`false`).
    /// If port security is enabled for the port,
    /// security group rules and anti-spoofing rules are applied to
    /// the traffic on the port. If disabled, no such rules are applied.
    #[serde()]
    #[structable(optional, wide)]
    port_security_enabled: Option<BoolString>,

    /// The ID of the QoS policy associated with the port.
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The ID of the QoS policy of the network where this port is plugged.
    #[serde()]
    #[structable(optional, wide)]
    qos_network_policy_id: Option<String>,

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

    /// The uplink status propagation of the port. Valid values are
    /// enabled (`true`) and disabled (`false`).
    #[serde()]
    #[structable(optional, wide)]
    propagate_uplink_status: Option<BoolString>,

    /// A valid DNS name.
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// Data assigned to a port by the Networking internal DNS including the
    /// `hostname`, `ip\_address` and `fqdn`.
    #[serde()]
    #[structable(optional, wide)]
    dns_assignment: Option<String>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The IDs of security groups applied to the port.
    #[serde()]
    #[structable(optional, wide)]
    security_groups: Option<VecString>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFixedIps {
    ip_address: Option<String>,
    subnet_id: Option<String>,
}

impl fmt::Display for ResponseFixedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "subnet_id={}",
                self.subnet_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        return write!(f, "{}", data.join(";"));
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFixedIps(Vec<ResponseFixedIps>);
impl fmt::Display for VecResponseFixedIps {
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
struct ResponseAllowedAddressPairs {
    ip_address: Option<String>,
    max_address: Option<String>,
}

impl fmt::Display for ResponseAllowedAddressPairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "max_address={}",
                self.max_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        return write!(f, "{}", data.join(";"));
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAllowedAddressPairs(Vec<ResponseAllowedAddressPairs>);
impl fmt::Display for VecResponseAllowedAddressPairs {
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecHashMapStringValue(Vec<HashMapStringValue>);
impl fmt::Display for VecHashMapStringValue {
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

#[async_trait]
impl Command for PortCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Port with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = create::Request::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters

        // Set Request.port data
        let args = &self.args.port;
        let mut port_builder = create::PortBuilder::default();
        if let Some(val) = &args.name {
            port_builder.name(val);
        }

        if let Some(val) = &args.network_id {
            port_builder.network_id(val);
        }

        if let Some(val) = &args.admin_state_up {
            port_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.mac_address {
            port_builder.mac_address(val);
        }

        if let Some(val) = &args.fixed_ips {
            let sub: Vec<create::FixedIps> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::FixedIps>(v.clone()))
                .collect::<Vec<create::FixedIps>>();
            port_builder.fixed_ips(sub);
        }

        if let Some(val) = &args.device_id {
            port_builder.device_id(val);
        }

        if let Some(val) = &args.device_owner {
            port_builder.device_owner(val);
        }

        if let Some(val) = &args.tenant_id {
            port_builder.tenant_id(val);
        }

        if let Some(val) = &args.allowed_address_pairs {
            let sub: Vec<create::AllowedAddressPairs> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::AllowedAddressPairs>(v.clone()))
                .collect::<Vec<create::AllowedAddressPairs>>();
            port_builder.allowed_address_pairs(sub);
        }

        if let Some(val) = &args.extra_dhcp_opts {
            use std::collections::BTreeMap;
            port_builder.extra_dhcp_opts(
                val.iter()
                    .map(|v| {
                        v.as_object()
                            .expect("Is a valid Json object")
                            .iter()
                            .map(|(k, v)| (k.clone().into(), v.clone().into()))
                            .collect::<BTreeMap<_, Value>>()
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(val) = &args.device_profile {
            port_builder.device_profile(Some(val.into()));
        }

        if let Some(val) = &args.hints {
            port_builder.hints(val.iter().cloned());
        }

        if let Some(val) = &args.numa_affinity_policy {
            let tmp = match val {
                NumaAffinityPolicy::Preferred => create::NumaAffinityPolicy::Preferred,
                NumaAffinityPolicy::Required => create::NumaAffinityPolicy::Required,
                NumaAffinityPolicy::Legacy => create::NumaAffinityPolicy::Legacy,
            };
            port_builder.numa_affinity_policy(tmp);
        }

        if let Some(val) = &args.binding_vnic_type {
            let tmp = match val {
                BindingVnicType::RemoteManaged => create::BindingVnicType::RemoteManaged,
                BindingVnicType::Macvtap => create::BindingVnicType::Macvtap,
                BindingVnicType::DirectPhysical => create::BindingVnicType::DirectPhysical,
                BindingVnicType::AcceleratorDirect => create::BindingVnicType::AcceleratorDirect,
                BindingVnicType::Baremetal => create::BindingVnicType::Baremetal,
                BindingVnicType::AcceleratorDirectPhysical => {
                    create::BindingVnicType::AcceleratorDirectPhysical
                }
                BindingVnicType::Direct => create::BindingVnicType::Direct,
                BindingVnicType::Normal => create::BindingVnicType::Normal,
                BindingVnicType::VirtioForwarder => create::BindingVnicType::VirtioForwarder,
                BindingVnicType::Vdpa => create::BindingVnicType::Vdpa,
                BindingVnicType::SmartNic => create::BindingVnicType::SmartNic,
            };
            port_builder.binding_vnic_type(tmp);
        }

        if let Some(val) = &args.binding_host_id {
            port_builder.binding_host_id(val);
        }

        if let Some(val) = &args.binding_profile {
            port_builder.binding_profile(val.iter().cloned());
        }

        if let Some(val) = &args.port_security_enabled {
            port_builder.port_security_enabled(*val);
        }

        if let Some(val) = &args.qos_policy_id {
            port_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.tags {
            // None
            port_builder.tags(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.propagate_uplink_status {
            port_builder.propagate_uplink_status(*val);
        }

        if let Some(val) = &args.dns_name {
            port_builder.dns_name(val);
        }

        if let Some(val) = &args.dns_domain {
            port_builder.dns_domain(val);
        }

        if let Some(val) = &args.description {
            port_builder.description(val);
        }

        if let Some(val) = &args.security_groups {
            // None
            port_builder.security_groups(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        ep_builder.port(port_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
