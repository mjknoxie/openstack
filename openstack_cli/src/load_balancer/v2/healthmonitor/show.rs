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

//! Show Healthmonitor command
//!
//! Wraps invoking of the `v2/lbaas/healthmonitors/{healthmonitor_id}` with `GET` method

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

use openstack_sdk::api::find;
use openstack_sdk::api::load_balancer::v2::healthmonitor::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows the details of a health monitor.
///
/// If you are not an administrative user and the parent load balancer does not
/// belong to your project, the service returns the HTTP `Forbidden (403)`
/// response code.
///
/// This operation does not require a request body.
///
#[derive(Args)]
#[command(about = "Show Health Monitor details")]
pub struct HealthmonitorCommand {
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
    /// healthmonitor_id parameter for
    /// /v2/lbaas/healthmonitors/{healthmonitor_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Healthmonitor response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The time, in seconds, between sending probes to members.
    ///
    #[serde()]
    #[structable(optional)]
    delay: Option<i32>,

    /// The domain name, which be injected into the HTTP Host Header to the
    /// backend server for HTTP health check.
    ///
    /// **New in version 2.10**
    ///
    #[serde()]
    #[structable(optional)]
    domain_name: Option<String>,

    /// The list of HTTP status codes expected in response from the member to
    /// declare it healthy. Specify one of the following values:
    ///
    /// - A single value, such as `200`
    /// - A list, such as `200, 202`
    /// - A range, such as `200-204`
    ///
    #[serde()]
    #[structable(optional)]
    expected_codes: Option<String>,

    /// The HTTP method that the health monitor uses for requests. One of
    /// `CONNECT`, `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT`,
    /// or `TRACE`.
    ///
    #[serde()]
    #[structable(optional)]
    http_method: Option<String>,

    /// The HTTP version. One of `1.0` or `1.1`. The default is `1.0`.
    ///
    /// **New in version 2.10**
    ///
    #[serde()]
    #[structable(optional)]
    http_version: Option<f32>,

    /// The associated health monitor ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The number of successful checks before changing the `operating status`
    /// of the member to `ONLINE`. A valid value is from `1` to `10`.
    ///
    #[serde()]
    #[structable(optional)]
    max_retries: Option<i32>,

    /// The number of allowed check failures before changing the
    /// `operating status` of the member to `ERROR`. A valid value is from `1`
    /// to `10`.
    ///
    #[serde()]
    #[structable(optional)]
    max_retries_down: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    pools: Option<Value>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional)]
    provisioning_status: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The maximum time, in seconds, that a monitor waits to connect before it
    /// times out. This value must be less than the delay value.
    ///
    #[serde()]
    #[structable(optional)]
    timeout: Option<i32>,

    /// The type of health monitor. One of `HTTP`, `HTTPS`, `PING`, `SCTP`,
    /// `TCP`, `TLS-HELLO`, or `UDP-CONNECT`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The HTTP URL path of the request sent by the monitor to test the health
    /// of a backend member. Must be a string that begins with a forward slash
    /// (`/`).
    ///
    #[serde()]
    #[structable(optional)]
    url_path: Option<String>,
}

impl HealthmonitorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Healthmonitor");

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
