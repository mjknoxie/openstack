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

//! Show Rule command
//!
//! Wraps invoking of the `v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id}` with `GET` method

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

use openstack_sdk::api::load_balancer::v2::l7policy::rule::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows the details of a L7 rule.
///
/// If you are not an administrative user and the L7 rule object does not
/// belong to your project, the service returns the HTTP `Forbidden (403)`
/// response code.
///
/// This operation does not require a request body.
///
#[derive(Args)]
#[command(about = "Show L7 Rule details")]
pub struct RuleCommand {
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
    /// l7policy_id parameter for
    /// /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_l7policy_id",
        value_name = "L7POLICY_ID"
    )]
    l7policy_id: String,

    /// rule_id parameter for
    /// /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Rule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`,
    /// `EQUAL_TO`, `REGEX`, or `STARTS_WITH`.
    ///
    #[serde()]
    #[structable(optional)]
    compare_type: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The ID of the L7 rule.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// When `true` the logic of the rule is inverted. For example, with invert
    /// `true`, equal to would become not equal to.
    ///
    #[serde()]
    #[structable(optional)]
    invert: Option<bool>,

    /// The key to use for the comparison. For example, the name of the cookie
    /// to evaluate.
    ///
    #[serde()]
    #[structable(optional)]
    key: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

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

    /// The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`,
    /// `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The value to use for the comparison. For example, the file type to
    /// compare.
    ///
    #[serde()]
    #[structable(optional)]
    value: Option<String>,
}

impl RuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Rule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.l7policy_id(&self.path.l7policy_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
