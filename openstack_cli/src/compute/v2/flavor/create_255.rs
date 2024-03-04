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

//! Create Flavor command [microversion = 2.55]
//!
//! Wraps invoking of the `v2.1/flavors` with `POST` method

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

use crate::common::IntString;
use crate::common::NumString;
use openstack_sdk::api::compute::v2::flavor::create_255;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a flavor.
///
/// Creating a flavor is typically only available to administrators of a cloud
/// because this has implications for scheduling efficiently in the cloud.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
///
#[derive(Args)]
#[command(about = "Create Flavor (microversion = 2.55)")]
pub struct FlavorCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    flavor: Flavor,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Flavor Body data
#[derive(Args)]
struct Flavor {
    /// The display name of a flavor.
    ///
    #[arg(long)]
    name: String,

    /// Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces
    /// and dots ‘.’ are permitted. If an ID is not provided, then a default
    /// UUID will be assigned.
    ///
    #[arg(long)]
    id: Option<String>,

    /// The number of virtual CPUs that will be allocated to the server.
    ///
    #[arg(long)]
    ram: String,

    /// The number of virtual CPUs that will be allocated to the server.
    ///
    #[arg(long)]
    vcpus: String,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    ///
    #[arg(long)]
    disk: String,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    ///
    #[arg(long)]
    os_flv_ext_data_ephemeral: Option<String>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    ///
    #[arg(long)]
    swap: Option<String>,

    /// The receive / transmit factor (as a float) that will be set on ports if
    /// the network backend supports the QOS extension. Otherwise it will be
    /// ignored. It defaults to 1.0.
    ///
    #[arg(long)]
    rxtx_factor: Option<String>,

    /// Whether the flavor is public (available to all projects) or scoped to a
    /// set of projects. Default is True if not specified.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    os_flavor_access_is_public: Option<bool>,

    /// A free form description of the flavor. Limited to 65535 characters in
    /// length. Only printable characters are allowed.
    ///
    /// **New in version 2.55**
    ///
    #[arg(long)]
    description: Option<String>,
}

/// Flavor response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The display name of a flavor.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The amount of RAM a flavor has, in MiB.
    ///
    #[serde()]
    #[structable(optional)]
    ram: Option<IntString>,

    /// The number of virtual CPUs that will be allocated to the server.
    ///
    #[serde()]
    #[structable(optional)]
    vcpus: Option<IntString>,

    /// The size of the root disk that will be created in GiB. If 0 the root
    /// disk will be set to exactly the size of the image used to deploy the
    /// instance. However, in this case the scheduler cannot select the compute
    /// host based on the virtual image size. Therefore, 0 should only be used
    /// for volume booted instances or for testing purposes. Volume-backed
    /// instances can be enforced for flavors with zero root disk via the
    /// `os_compute_api:servers:create:zero_disk_flavor` policy rule.
    ///
    #[serde()]
    #[structable(optional)]
    disk: Option<IntString>,

    /// The size of the ephemeral disk that will be created, in GiB. Ephemeral
    /// disks may be written over on server state changes. So should only be
    /// used as a scratch space for applications that are aware of its
    /// limitations. Defaults to 0.
    ///
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    #[structable(optional, title = "OS-FLV-EXT-DATA:ephemeral")]
    os_flv_ext_data_ephemeral: Option<IntString>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created. Currently, the
    /// empty string (‘’) is used to represent 0. As of microversion 2.75
    /// default return value of swap is 0 instead of empty string.
    ///
    #[serde()]
    #[structable(optional)]
    swap: Option<IntString>,

    /// The receive / transmit factor (as a float) that will be set on ports if
    /// the network backend supports the QOS extension. Otherwise it will be
    /// ignored. It defaults to 1.0.
    ///
    #[serde()]
    #[structable(optional)]
    rxtx_factor: Option<NumString>,

    /// Whether the flavor is public (available to all projects) or scoped to a
    /// set of projects. Default is True if not specified.
    ///
    #[serde(rename = "os-flavor-access:is_public")]
    #[structable(optional, title = "os-flavor-access:is_public")]
    os_flavor_access_is_public: Option<bool>,

    /// A dictionary of the flavor’s extra-specs key-and-value pairs. This will
    /// only be included if the user is allowed by policy to index flavor
    /// extra_specs.
    ///
    /// **New in version 2.61**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    extra_specs: Option<Value>,

    /// Links to the resources in question. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Flavor");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_255::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.55");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.flavor data
        let args = &self.flavor;
        let mut flavor_builder = create_255::FlavorBuilder::default();

        flavor_builder.name(&args.name);

        if let Some(val) = &args.id {
            flavor_builder.id(Some(val.into()));
        }

        flavor_builder.ram(&args.ram);

        flavor_builder.vcpus(&args.vcpus);

        flavor_builder.disk(&args.disk);

        if let Some(val) = &args.os_flv_ext_data_ephemeral {
            flavor_builder.os_flv_ext_data_ephemeral(val);
        }

        if let Some(val) = &args.swap {
            flavor_builder.swap(val);
        }

        if let Some(val) = &args.rxtx_factor {
            flavor_builder.rxtx_factor(val);
        }

        if let Some(val) = &args.os_flavor_access_is_public {
            flavor_builder.os_flavor_access_is_public(*val);
        }

        if let Some(val) = &args.description {
            flavor_builder.description(Some(val.into()));
        }

        ep_builder.flavor(flavor_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
