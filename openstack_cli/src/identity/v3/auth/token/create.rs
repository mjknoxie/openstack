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

//! Create Token command
//!
//! Wraps invoking of the `v3/auth/tokens` with `POST` method

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

use clap::ValueEnum;
use dialoguer::Password;
use openstack_sdk::api::identity::v3::auth::token::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Authenticates an identity and generates a token. Uses the password
/// authentication method. Authorization is unscoped.
///
/// The request body must include a payload that specifies the authentication
/// method, which is `password`, and the user, by ID or name, and password
/// credentials.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`
///
#[derive(Args)]
#[command(about = "Password authentication with unscoped authorization")]
pub struct TokenCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// An `auth` object.
    ///
    #[command(flatten)]
    auth: Auth,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Methods {
    ApplicationCredential,
    Password,
    Token,
    Totp,
}

/// Domain Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Domain {
    /// User Domain ID
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// User Domain Name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// User Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct User {
    /// A `domain` object
    ///
    #[command(flatten)]
    domain: Option<Domain>,

    /// The ID of the user. Required if you do not specify the user name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The user name. Required if you do not specify the ID of the user. If
    /// you specify the user name, you must also specify the domain, by ID or
    /// name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// User Password
    ///
    #[arg(help_heading = "Body parameters", long)]
    password: Option<String>,
}

/// Password Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Password {
    /// A `user` object.
    ///
    #[command(flatten)]
    user: Option<User>,
}

/// Token Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Token {
    /// Authorization Token value
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    id: Option<String>,
}

/// UserDomainStructInput Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct UserDomainStructInput {
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// TotpUser Body data
#[derive(Args)]
#[group(required = true, multiple = true)]
struct TotpUser {
    #[command(flatten)]
    domain: Option<UserDomainStructInput>,

    /// The user ID
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The user name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// MFA passcode
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    passcode: Option<String>,
}

/// Totp Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Totp {
    #[command(flatten)]
    user: TotpUser,
}

/// ApplicationCredentialUser Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct ApplicationCredentialUser {
    #[command(flatten)]
    domain: Option<UserDomainStructInput>,

    /// The user ID
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// The user name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// ApplicationCredential Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct ApplicationCredential {
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The secret for authenticating the application credential.
    ///
    #[arg(help_heading = "Body parameters", long, required = false)]
    secret: Option<String>,

    /// A user object, required if an application credential is identified by
    /// name and not ID.
    ///
    #[command(flatten)]
    user: Option<ApplicationCredentialUser>,
}

/// Identity Body data
#[derive(Args)]
#[group(required = true, multiple = true)]
struct Identity {
    /// An application credential object.
    ///
    #[command(flatten)]
    application_credential: Option<ApplicationCredential>,

    /// The authentication method. For password authentication, specify
    /// `password`.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, required=false)]
    methods: Vec<Methods>,

    /// The `password` object, contains the authentication information.
    ///
    #[command(flatten)]
    password: Option<Password>,

    /// A `token` object
    ///
    #[command(flatten)]
    token: Option<Token>,

    /// Multi Factor Authentication information
    ///
    #[command(flatten)]
    totp: Option<Totp>,
}

/// ProjectDomain Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct ProjectDomain {
    /// Project domain Id
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// Project domain name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// Project Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Project {
    #[command(flatten)]
    domain: Option<ProjectDomain>,

    /// Project Id
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// Project Name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// ScopeDomain Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct ScopeDomain {
    /// Domain id
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// Domain name
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// OsTrustTrust Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct OsTrustTrust {
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,
}

/// System Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct System {
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    all: Option<bool>,
}

/// Scope Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Scope {
    #[command(flatten)]
    domain: Option<ScopeDomain>,

    #[command(flatten)]
    os_trust_trust: Option<OsTrustTrust>,

    #[command(flatten)]
    project: Option<Project>,

    #[command(flatten)]
    system: Option<System>,
}

/// Auth Body data
#[derive(Args)]
struct Auth {
    /// An `identity` object.
    ///
    #[command(flatten)]
    identity: Identity,

    /// The authorization scope, including the system (Since v3.10), a project,
    /// or a domain (Since v3.4). If multiple scopes are specified in the same
    /// request (e.g. project and domain or domain and system) an HTTP 400 Bad
    /// Request will be returned, as a token cannot be simultaneously scoped to
    /// multiple authorization targets. An ID is sufficient to uniquely
    /// identify a project but if a project is specified by name, then the
    /// domain of the project must also be specified in order to uniquely
    /// identify the project by name. A domain scope may be specified by either
    /// the domain’s ID or name with equivalent results.
    ///
    #[command(flatten)]
    scope: Option<Scope>,
}

/// Token response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of one or two audit IDs. An audit ID is a unique, randomly
    /// generated, URL-safe string that you can use to track a token. The first
    /// audit ID is the current audit ID for the token. The second audit ID is
    /// present for only re-scoped tokens and is the audit ID from the token
    /// before it was re-scoped. A re- scoped token is one that was exchanged
    /// for another token of the same or different scope. You can use these
    /// audit IDs to track the use of a token or chain of tokens across
    /// multiple requests and endpoints without exposing the token ID to
    /// non-privileged users.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    audit_ids: Option<Value>,

    /// A `catalog` object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    catalog: Option<Value>,

    /// A domain object including the id and name representing the domain the
    /// token is scoped to. This is only included in tokens that are scoped to
    /// a domain.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    domain: Option<Value>,

    /// The date and time when the token expires.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss.sssZ
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58.000000Z`.
    ///
    /// A `null` value indicates that the token never expires.
    ///
    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The date and time when the token was issued.
    ///
    #[serde()]
    #[structable(optional)]
    issues_at: Option<String>,

    /// The authentication methods, which are commonly `password`, `token`, or
    /// other methods. Indicates the accumulated set of authentication methods
    /// that were used to obtain the token. For example, if the token was
    /// obtained by password authentication, it contains `password`. Later, if
    /// the token is exchanged by using the token authentication method one or
    /// more times, the subsequently created tokens contain both `password` and
    /// `token` in their `methods` attribute. Unlike multi-factor
    /// authentication, the `methods` attribute merely indicates the methods
    /// that were used to authenticate the user in exchange for a token. The
    /// client is responsible for determining the total number of
    /// authentication factors.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    methods: Option<Value>,

    /// A `project` object including the `id`, `name` and `domain` object
    /// representing the project the token is scoped to. This is only included
    /// in tokens that are scoped to a project.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    project: Option<Value>,

    /// A list of `role` objects
    ///
    #[serde()]
    #[structable(optional, pretty)]
    roles: Option<Value>,

    /// A `system` object containing information about which parts of the
    /// system the token is scoped to. If the token is scoped to the entire
    /// deployment system, the `system` object will consist of `{"all": true}`.
    /// This is only included in tokens that are scoped to the system.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    system: Option<Value>,

    /// A `user` object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    user: Option<Value>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
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
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseUser {
    domain: Option<Value>,
    id: Option<String>,
    name: Option<String>,
    os_federation: Option<Value>,
    password_expires_at: Option<String>,
}

impl fmt::Display for ResponseUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "domain={}",
                self.domain
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
                "name={}",
                self.name
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
            format!(
                "password_expires_at={}",
                self.password_expires_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
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
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
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

impl TokenCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Token");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.auth data
        let args = &self.auth;
        let mut auth_builder = create::AuthBuilder::default();

        let mut identity_builder = create::IdentityBuilder::default();

        identity_builder.methods(
            args.identity
                .methods
                .iter()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
        );
        if let Some(val) = &&args.identity.password {
            let mut password_builder = create::PasswordBuilder::default();
            if let Some(val) = &val.user {
                let mut user_builder = create::UserBuilder::default();
                if let Some(val) = &val.id {
                    user_builder.id(val);
                }
                if let Some(val) = &val.name {
                    user_builder.name(val);
                }
                if let Some(val) = &val.password {
                    user_builder.password(val);
                }
                if let Some(val) = &val.domain {
                    let mut domain_builder = create::DomainBuilder::default();
                    if let Some(val) = &val.id {
                        domain_builder.id(val);
                    }
                    if let Some(val) = &val.name {
                        domain_builder.name(val);
                    }
                    user_builder.domain(domain_builder.build().expect("A valid object"));
                }
                password_builder.user(user_builder.build().expect("A valid object"));
            }
            identity_builder.password(password_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.token {
            let mut token_builder = create::TokenBuilder::default();

            token_builder.id(&val.id);
            identity_builder.token(token_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.totp {
            let mut totp_builder = create::TotpBuilder::default();

            let mut user_builder = create::TotpUserBuilder::default();
            if let Some(val) = &&val.user.id {
                user_builder.id(val);
            }
            if let Some(val) = &&val.user.name {
                user_builder.name(val);
            }
            if let Some(val) = &&val.user.domain {
                let mut domain_builder = create::UserDomainStructInputBuilder::default();
                if let Some(val) = &val.id {
                    domain_builder.id(val);
                }
                if let Some(val) = &val.name {
                    domain_builder.name(val);
                }
                user_builder.domain(domain_builder.build().expect("A valid object"));
            }

            user_builder.passcode(&val.user.passcode);
            totp_builder.user(user_builder.build().expect("A valid object"));
            identity_builder.totp(totp_builder.build().expect("A valid object"));
        }
        if let Some(val) = &&args.identity.application_credential {
            let mut application_credential_builder =
                create::ApplicationCredentialBuilder::default();
            if let Some(val) = &val.id {
                application_credential_builder.id(val);
            }
            if let Some(val) = &val.name {
                application_credential_builder.name(val);
            }

            application_credential_builder.secret(&val.secret);
            if let Some(val) = &val.user {
                let mut user_builder = create::ApplicationCredentialUserBuilder::default();
                if let Some(val) = &val.id {
                    user_builder.id(val);
                }
                if let Some(val) = &val.name {
                    user_builder.name(val);
                }
                if let Some(val) = &val.domain {
                    let mut domain_builder = create::UserDomainStructInputBuilder::default();
                    if let Some(val) = &val.id {
                        domain_builder.id(val);
                    }
                    if let Some(val) = &val.name {
                        domain_builder.name(val);
                    }
                    user_builder.domain(domain_builder.build().expect("A valid object"));
                }
                application_credential_builder.user(user_builder.build().expect("A valid object"));
            }
            identity_builder.application_credential(
                application_credential_builder
                    .build()
                    .expect("A valid object"),
            );
        }
        auth_builder.identity(identity_builder.build().expect("A valid object"));

        if let Some(val) = &args.scope {
            let mut scope_builder = create::ScopeBuilder::default();
            if let Some(val) = &val.project {
                let mut project_builder = create::ProjectBuilder::default();
                if let Some(val) = &val.name {
                    project_builder.name(val);
                }
                if let Some(val) = &val.id {
                    project_builder.id(val);
                }
                if let Some(val) = &val.domain {
                    let mut domain_builder = create::ProjectDomainBuilder::default();
                    if let Some(val) = &val.id {
                        domain_builder.id(val);
                    }
                    if let Some(val) = &val.name {
                        domain_builder.name(val);
                    }
                    project_builder.domain(domain_builder.build().expect("A valid object"));
                }
                scope_builder.project(project_builder.build().expect("A valid object"));
            }
            if let Some(val) = &val.domain {
                let mut domain_builder = create::ScopeDomainBuilder::default();
                if let Some(val) = &val.id {
                    domain_builder.id(val);
                }
                if let Some(val) = &val.name {
                    domain_builder.name(val);
                }
                scope_builder.domain(domain_builder.build().expect("A valid object"));
            }
            if let Some(val) = &val.os_trust_trust {
                let mut os_trust_trust_builder = create::OsTrustTrustBuilder::default();
                if let Some(val) = &val.id {
                    os_trust_trust_builder.id(val);
                }
                scope_builder
                    .os_trust_trust(os_trust_trust_builder.build().expect("A valid object"));
            }
            if let Some(val) = &val.system {
                let mut system_builder = create::SystemBuilder::default();
                if let Some(val) = &val.all {
                    system_builder.all(*val);
                }
                scope_builder.system(system_builder.build().expect("A valid object"));
            }
            auth_builder.scope(scope_builder.build().expect("A valid object"));
        }

        ep_builder.auth(auth_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
