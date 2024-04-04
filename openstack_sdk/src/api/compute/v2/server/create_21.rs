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

//! Creates a server.
//!
//! The progress of this operation depends on the location of the requested
//! image, network I/O, host load, selected flavor, and other factors.
//!
//! To check the progress of the request, make a `GET /servers/{id}` request.
//! This call returns a progress attribute, which is a percentage value from 0
//! to 100.
//!
//! The `Location` header returns the full URL to the newly created server and
//! is available as a `self` and `bookmark` link in the server representation.
//!
//! When you create a server, the response shows only the server ID, its links,
//! and the admin password. You can get additional attributes through
//! subsequent `GET` requests on the server.
//!
//! Include the `block_device_mapping_v2` parameter in the create request body
//! to boot a server from a volume.
//!
//! Include the `key_name` parameter in the create request body to add a
//! keypair to the server when you create it. To create a keypair, make a
//! [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair)
//! request.
//!
//! **Preconditions**
//!
//! **Asynchronous postconditions**
//!
//! **Troubleshooting**
//!
//! Normal response codes: 202
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404), conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Networks<'a> {
    /// Schedule the server on a host in the network specified with this
    /// parameter and a cidr (`os:scheduler_hints.cidr`). It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) fixed_ip: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) port: Option<Option<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) uuid: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum OsDcfDiskConfig {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Personality<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) contents: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) path: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct BlockDeviceMapping<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) connection_info: Option<Cow<'a, str>>,

    /// Indicates whether a config drive enables metadata injection. The
    /// config_drive setting provides information about a drive that the
    /// instance can mount at boot time. The instance reads files from the
    /// drive to get information that is normally available through the
    /// metadata service. This metadata is different from the user data. Not
    /// all cloud providers enable the `config_drive`. Read more in the
    /// [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) delete_on_termination: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) no_device: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) snapshot_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) virtual_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) volume_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) volume_size: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum SourceType {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "volume")]
    Volume,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum DestinationType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "volume")]
    Volume,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct BlockDeviceMappingV2<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) boot_index: Option<Option<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) connection_info: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) delete_on_termination: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) destination_type: Option<DestinationType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_type: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) disk_bus: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) guest_format: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) image_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) no_device: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) snapshot_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) source_type: Option<SourceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) uuid: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) virtual_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) volume_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) volume_size: Option<i32>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct SecurityGroups<'a> {
    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

/// A `server` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Server<'a> {
    /// IPv4 address that should be used to access this server.
    ///
    #[serde(rename = "accessIPv4", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) access_ipv4: Option<Cow<'a, str>>,

    /// IPv6 address that should be used to access this server.
    ///
    #[serde(rename = "accessIPv6", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) access_ipv6: Option<Cow<'a, str>>,

    /// The administrative password of the server. If you omit this parameter,
    /// the operation generates a new password.
    ///
    #[serde(rename = "adminPass", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) admin_pass: Option<Cow<'a, str>>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) availability_zone: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) block_device_mapping: Option<Vec<BlockDeviceMapping<'a>>>,

    /// Enables fine grained control of the block device mapping for an
    /// instance. This is typically used for booting servers from volumes. An
    /// example format would look as follows:
    ///
    /// > ```text
    /// > "block_device_mapping_v2": [{
    /// >     "boot_index": "0",
    /// >     "uuid": "ac408821-c95a-448f-9292-73986c790911",
    /// >     "source_type": "image",
    /// >     "volume_size": "25",
    /// >     "destination_type": "volume",
    /// >     "delete_on_termination": true,
    /// >     "tag": "disk1",
    /// >     "disk_bus": "scsi"}]
    /// >
    /// > ```
    ///
    /// In microversion 2.32, `tag` is an optional string attribute that can be
    /// used to assign a tag to the block device. This tag is then exposed to
    /// the guest in the metadata API and the config drive and is associated to
    /// hardware metadata for that block device, such as bus (ex: SCSI), bus
    /// address (ex: 1:0:2:0), and serial.
    ///
    /// A bug has caused the `tag` attribute to no longer be accepted starting
    /// with version 2.33. It has been restored in version 2.42.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) block_device_mapping_v2: Option<Vec<BlockDeviceMappingV2<'a>>>,

    /// Indicates whether a config drive enables metadata injection. The
    /// config_drive setting provides information about a drive that the
    /// instance can mount at boot time. The instance reads files from the
    /// drive to get information that is normally available through the
    /// metadata service. This metadata is different from the user data. Not
    /// all cloud providers enable the `config_drive`. Read more in the
    /// [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) config_drive: Option<bool>,

    /// The flavor reference, as an ID (including a UUID) or full URL, for the
    /// flavor for your server instance.
    ///
    #[serde(rename = "flavorRef")]
    #[builder(setter(into))]
    pub(crate) flavor_ref: Cow<'a, str>,

    /// The UUID of the image to use for your server instance. This is not
    /// required in case of boot from volume. In all other cases it is required
    /// and must be a valid UUID otherwise API will return 400.
    ///
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) image_ref: Option<Cow<'a, str>>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) key_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_count: Option<i32>,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_metadata"))]
    pub(crate) metadata: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) min_count: Option<i32>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// A list of `network` object. Required parameter when there are multiple
    /// networks defined for the tenant. When you do not specify the networks
    /// parameter, the server attaches to the only network created for the
    /// current tenant. Optionally, you can create one or more NICs on the
    /// server. To provision the server instance with a NIC for a network,
    /// specify the UUID of the network in the `uuid` attribute in a `networks`
    /// object. To provision the server instance with a NIC for an already
    /// existing port, specify the port-id in the `port` attribute in a
    /// `networks` object.
    ///
    /// If multiple networks are defined, the order in which they appear in the
    /// guest operating system will not necessarily reflect the order in which
    /// they are given in the server boot request. Guests should therefore not
    /// depend on device order to deduce any information about their network
    /// devices. Instead, device role tags should be used: introduced in 2.32,
    /// broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an
    /// optional, string attribute that can be used to assign a tag to a
    /// virtual network interface. This tag is then exposed to the guest in the
    /// metadata API and the config drive and is associated to hardware
    /// metadata for that network interface, such as bus (ex: PCI), bus address
    /// (ex: 0000:00:02.0), and MAC address.
    ///
    /// A bug has caused the `tag` attribute to no longer be accepted starting
    /// with version 2.37. Therefore, network interfaces could only be tagged
    /// in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the
    /// `tag` attribute.
    ///
    /// Starting with microversion 2.37, this field is required and the special
    /// string values *auto* and *none* can be specified for networks. *auto*
    /// tells the Compute service to use a network that is available to the
    /// project, if one exists. If one does not exist, the Compute service will
    /// attempt to automatically allocate a network for the project (if
    /// possible). *none* tells the Compute service to not allocate a network
    /// for the instance. The *auto* and *none* values cannot be used with any
    /// other network values, including other network uuids, ports, fixed IPs
    /// or device tags. These are requested as strings for the networks value,
    /// not in a list. See the associated example.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) networks: Option<Vec<Networks<'a>>>,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers. A server inherits the `OS-DCF:diskConfig` value from
    /// the image from which it was created, and an image inherits the
    /// `OS-DCF:diskConfig` value from the server from which it was created. To
    /// override the inherited setting, you can include this attribute in the
    /// request body of a server create, rebuild, or resize request. If the
    /// `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a
    /// server from that image and set its `OS-DCF:diskConfig` value to `AUTO`.
    /// A valid value is:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using whatever partition
    ///   scheme and file system is in the source image. If the target flavor
    ///   disk is larger, the API does not partition the remaining disk space.
    ///
    #[serde(rename = "OS-DCF:diskConfig", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// The file path and contents, text only, to inject into the server at
    /// launch. The maximum size of the file path data is 255 bytes. The
    /// maximum limit is the number of allowed bytes in the decoded, rather
    /// than encoded, data.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) personality: Option<Vec<Personality<'a>>>,

    /// Indicates whether a config drive enables metadata injection. The
    /// config_drive setting provides information about a drive that the
    /// instance can mount at boot time. The instance reads files from the
    /// drive to get information that is normally available through the
    /// metadata service. This metadata is different from the user data. Not
    /// all cloud providers enable the `config_drive`. Read more in the
    /// [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) return_reservation_id: Option<bool>,

    /// One or more security groups. Specify the name of the security group in
    /// the `name` attribute. If you omit this attribute, the API creates the
    /// server in the `default` security group. Requested security groups are
    /// not applied to pre-existing ports.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) security_groups: Option<Vec<SecurityGroups<'a>>>,

    /// Configuration information or scripts to use upon launch. Must be Base64
    /// encoded. Restricted to 65535 bytes.
    ///
    /// Note
    ///
    /// The `null` value allowed in Nova legacy v2 API, but due to the strict
    /// input validation, it isn’t allowed in Nova v2.1 API.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) user_data: Option<Cow<'a, str>>,
}

impl<'a> ServerBuilder<'a> {
    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.metadata
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

/// The dictionary of data to send to the scheduler. Alternatively, you can
/// specify `OS-SCH-HNT:scheduler_hints` as the key in the request body.
///
/// Note
///
/// This is a top-level key in the request body, not part of the server portion
/// of the request body.
///
/// There are a few caveats with scheduler hints:
///
/// - The request validation schema is per hint. For example, some require a
///   single string value, and some accept a list of values.
/// - Hints are only used based on the cloud scheduler configuration, which
///   varies per deployment.
/// - Hints are pluggable per deployment, meaning that a cloud can have custom
///   hints which may not be available in another cloud.
///
/// For these reasons, it is important to consult each cloud’s user
/// documentation to know what is available for scheduler hints.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct OsSchedulerHints<'a> {
    /// Schedule the server on a host in the network specified with this
    /// parameter and a cidr (`os:scheduler_hints.cidr`). It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) build_near_host_ip: Option<Cow<'a, str>>,

    /// Schedule the server on a host in the network specified with an IP
    /// address (`os:scheduler_hints:build_near_host_ip`) and this parameter.
    /// If `os:scheduler_hints:build_near_host_ip` is specified and this
    /// parameter is omitted, `/24` is used. It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) cidr: Option<Cow<'a, str>>,

    /// A list of cell routes or a cell route (string). Schedule the server in
    /// a cell that is not specified. It is available when
    /// `DifferentCellFilter` is available on cloud side that is cell v1
    /// environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) different_cell: Option<Vec<Cow<'a, str>>>,

    /// A list of server UUIDs or a server UUID. Schedule the server on a
    /// different host from a set of servers. It is available when
    /// `DifferentHostFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) different_host: Option<Vec<Cow<'a, str>>>,

    /// The server group UUID. Schedule the server according to a policy of the
    /// server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or
    /// `soft-affinity`). It is available when `ServerGroupAffinityFilter`,
    /// `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`,
    /// `ServerGroupSoftAffinityWeigher` are available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) group: Option<Cow<'a, str>>,

    /// Schedule the server by using a custom filter in JSON format. For
    /// example:
    ///
    /// ```text
    /// "query": "[\">=\",\"$free_ram_mb\",1024]"
    ///
    /// ```
    ///
    /// It is available when `JsonFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) query: Option<Value>,

    /// A list of server UUIDs or a server UUID. Schedule the server on the
    /// same host as another server in a set of servers. It is available when
    /// `SameHostFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) same_host: Option<Vec<Cow<'a, str>>>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) target_cell: Option<Cow<'a, str>>,

    #[builder(setter(name = "_properties"), default, private)]
    #[serde(flatten)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}

impl<'a> OsSchedulerHintsBuilder<'a> {
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct OsSchHntSchedulerHints<'a> {
    /// Schedule the server on a host in the network specified with this
    /// parameter and a cidr (`os:scheduler_hints.cidr`). It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) build_near_host_ip: Option<Cow<'a, str>>,

    /// Schedule the server on a host in the network specified with an IP
    /// address (`os:scheduler_hints:build_near_host_ip`) and this parameter.
    /// If `os:scheduler_hints:build_near_host_ip` is specified and this
    /// parameter is omitted, `/24` is used. It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) cidr: Option<Cow<'a, str>>,

    /// A list of cell routes or a cell route (string). Schedule the server in
    /// a cell that is not specified. It is available when
    /// `DifferentCellFilter` is available on cloud side that is cell v1
    /// environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) different_cell: Option<Vec<Cow<'a, str>>>,

    /// A list of server UUIDs or a server UUID. Schedule the server on a
    /// different host from a set of servers. It is available when
    /// `DifferentHostFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) different_host: Option<Vec<Cow<'a, str>>>,

    /// The server group UUID. Schedule the server according to a policy of the
    /// server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or
    /// `soft-affinity`). It is available when `ServerGroupAffinityFilter`,
    /// `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`,
    /// `ServerGroupSoftAffinityWeigher` are available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) group: Option<Cow<'a, str>>,

    /// Schedule the server by using a custom filter in JSON format. For
    /// example:
    ///
    /// ```text
    /// "query": "[\">=\",\"$free_ram_mb\",1024]"
    ///
    /// ```
    ///
    /// It is available when `JsonFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) query: Option<Value>,

    /// A list of server UUIDs or a server UUID. Schedule the server on the
    /// same host as another server in a set of servers. It is available when
    /// `SameHostFilter` is available on cloud side.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) same_host: Option<Vec<Cow<'a, str>>>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) target_cell: Option<Cow<'a, str>>,

    #[builder(setter(name = "_properties"), default, private)]
    #[serde(flatten)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}

impl<'a> OsSchHntSchedulerHintsBuilder<'a> {
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    pub(crate) os_sch_hnt_scheduler_hints: Option<OsSchHntSchedulerHints<'a>>,

    /// The dictionary of data to send to the scheduler. Alternatively, you can
    /// specify `OS-SCH-HNT:scheduler_hints` as the key in the request body.
    ///
    /// Note
    ///
    /// This is a top-level key in the request body, not part of the server
    /// portion of the request body.
    ///
    /// There are a few caveats with scheduler hints:
    ///
    /// - The request validation schema is per hint. For example, some require
    ///   a single string value, and some accept a list of values.
    /// - Hints are only used based on the cloud scheduler configuration, which
    ///   varies per deployment.
    /// - Hints are pluggable per deployment, meaning that a cloud can have
    ///   custom hints which may not be available in another cloud.
    ///
    /// For these reasons, it is important to consult each cloud’s user
    /// documentation to know what is available for scheduler hints.
    ///
    #[builder(default, setter(into))]
    pub(crate) os_scheduler_hints: Option<OsSchedulerHints<'a>>,

    /// A `server` object.
    ///
    #[builder(setter(into))]
    pub(crate) server: Server<'a>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Server.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v2.1/servers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("server", serde_json::to_value(&self.server)?);
        if let Some(val) = &self.os_scheduler_hints {
            params.push("os:scheduler_hints", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.os_sch_hnt_scheduler_hints {
            params.push("OS-SCH-HNT:scheduler_hints", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .server(
                    ServerBuilder::default()
                        .flavor_ref("foo")
                        .name("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .server(
                ServerBuilder::default()
                    .flavor_ref("foo")
                    .name("foo")
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v2.1/servers".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .server(
                ServerBuilder::default()
                    .flavor_ref("foo")
                    .name("foo")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v2.1/servers".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .server(
                ServerBuilder::default()
                    .flavor_ref("foo")
                    .name("foo")
                    .build()
                    .unwrap(),
            )
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
