//! Validates and shows information for a token, including its expiration date
//! and authorization scope.
//!
//! Pass your own token in the `X-Auth-Token` request header.
//!
//! Pass the token that you want to validate in the `X-Subject-Token`
//! request header.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/auth\_tokens`
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::identity::v3::auth::token::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct TokenArgs {
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
pub struct PathParameters {}

/// Token get command
pub struct TokenCmd {
    pub args: TokenArgs,
}
/// Token response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A list of one or two audit IDs. An audit ID is a
    /// unique, randomly generated, URL-safe string that you can use to
    /// track a token. The first audit ID is the current audit ID for the
    /// token. The second audit ID is present for only re-scoped tokens
    /// and is the audit ID from the token before it was re-scoped. A re-
    /// scoped token is one that was exchanged for another token of the
    /// same or different scope. You can use these audit IDs to track the
    /// use of a token or chain of tokens across multiple requests and
    /// endpoints without exposing the token ID to non-privileged users.
    #[serde()]
    #[structable(optional)]
    audit_ids: Option<VecString>,

    /// A `catalog` object.
    #[serde()]
    #[structable(optional)]
    catalog: Option<VecResponseCatalog>,

    /// The date and time when the token expires.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss.sssZ
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58.000000Z`.
    ///
    ///
    /// A `null` value indicates that the token never expires.
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    /// The date and time when the token was issued.
    #[serde()]
    #[structable(optional)]
    issues_at: Option<String>,

    /// The authentication methods, which are commonly `password`,
    /// `token`, or other methods. Indicates the accumulated set of
    /// authentication methods that were used to obtain the token. For
    /// example, if the token was obtained by password authentication, it
    /// contains `password`. Later, if the token is exchanged by using
    /// the token authentication method one or more times, the
    /// subsequently created tokens contain both `password` and
    /// `token` in their `methods` attribute. Unlike multi-factor
    /// authentication, the `methods` attribute merely indicates the
    /// methods that were used to authenticate the user in exchange for a
    /// token. The client is responsible for determining the total number
    /// of authentication factors.
    #[serde()]
    #[structable(optional)]
    methods: Option<VecString>,

    /// A `user` object.
    #[serde()]
    #[structable(optional)]
    user: Option<ResponseUser>,

    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// A domain object including the id and name representing the domain the
    /// token is scoped to. This is only included in tokens that are scoped to
    /// a domain.
    #[serde()]
    #[structable(optional)]
    domain: Option<ResponseDomainStructResponse>,

    /// A `project` object
    #[serde()]
    #[structable(optional)]
    project: Option<ResponseProject>,

    /// A list of `role` objects
    #[serde()]
    #[structable(optional)]
    roles: Option<VecResponseRoles>,

    /// A `system` object containing information about which parts of the
    /// system
    /// the token is scoped to. If the token is scoped to the entire deployment
    /// system, the `system` object will consist of `{"all": true}`. This is
    /// only included in tokens that are scoped to the system.
    #[serde()]
    #[structable(optional)]
    system: Option<HashMapStringbool>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseEndpoints {
    id: Option<String>,
    interface: Option<String>,
    region: Option<String>,
    url: Option<String>,
}

impl fmt::Display for ResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "interface={}",
                self.interface
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "region={}",
                self.region
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "url={}",
                self.url
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseEndpoints(Vec<ResponseEndpoints>);
impl fmt::Display for VecResponseEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseCatalog {
    endpoints: Option<VecResponseEndpoints>,
    id: Option<String>,
    _type: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "endpoints={}",
                self.endpoints
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "_type={}",
                self._type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseCatalog(Vec<ResponseCatalog>);
impl fmt::Display for VecResponseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseDomain {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseUser {
    id: Option<String>,
    name: Option<String>,
    domain: Option<ResponseDomain>,
    password_expires_at: Option<String>,
    os_federation: Option<HashMapStringValue>,
}

impl fmt::Display for ResponseUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "domain={}",
                self.domain
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "password_expires_at={}",
                self.password_expires_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "os_federation={}",
                self.os_federation
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseDomainStructResponse {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseDomainStructResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseProject {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseRoles {
    id: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseRoles(Vec<ResponseRoles>);
impl fmt::Display for VecResponseRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringbool(HashMap<String, bool>);
impl fmt::Display for HashMapStringbool {
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

#[async_trait]
impl OSCCommand for TokenCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Token with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
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