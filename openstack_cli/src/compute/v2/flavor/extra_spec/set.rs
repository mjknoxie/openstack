//! Updates an extra spec, by key, for a flavor, by ID.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403)
//! itemNotFound(404), conflict(409)
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

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_key_val;

use openstack_sdk::api::compute::v2::flavor::extra_spec::set;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ExtraSpecArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, String>)]
    properties: Option<Vec<(String, String)>>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API
    #[arg()]
    flavor_id: String,

    /// id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
    #[arg()]
    id: String,
}

/// ExtraSpec set command
pub struct ExtraSpecCmd {
    pub args: ExtraSpecArgs,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResponseData(HashMap<String, serde_json::Value>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

#[async_trait]
impl OSCCommand for ExtraSpecCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ExtraSpec with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.flavor_id(&self.args.path.flavor_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}