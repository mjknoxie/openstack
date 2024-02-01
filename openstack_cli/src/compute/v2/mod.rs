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

pub mod aggregate;
pub mod availability_zone;
pub mod extension;
pub mod flavor;
pub mod hypervisor;
pub mod keypair;
pub mod server;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::compute::v2::aggregate::{AggregateArgs, AggregateCommand};
use crate::compute::v2::availability_zone::{AvailabilityZoneArgs, AvailabilityZoneCommand};
use crate::compute::v2::extension::{ExtensionArgs, ExtensionCommand};
use crate::compute::v2::flavor::{FlavorArgs, FlavorCommand};
use crate::compute::v2::hypervisor::{HypervisorArgs, HypervisorCommand};
use crate::compute::v2::keypair::{KeypairArgs, KeypairCommand};
use crate::compute::v2::server::{ServerArgs, ServerCommand};
use crate::{OSCCommand, OpenStackCliError};

/// Compute service (Nova) arguments
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ComputeSrvArgs {
    // /// Compute API microversion
    // #[arg(long, env = "OS_COMPUTE_API_VERSION")]
    // os_compute_api_version: Option<String>,
    /// Compute service resource
    #[command(subcommand)]
    command: ComputeSrvCommands,
}

/// Compute resources commands
#[derive(Clone, Subcommand)]
pub enum ComputeSrvCommands {
    #[command(about = "Host Aggregates")]
    Aggregate(Box<AggregateArgs>),
    /// Lists and gets detailed availability zone information.
    ///
    /// An availability zone is created or updated by setting the
    /// availability_zone parameter in the create, update, or
    /// create or update methods of the Host Aggregates API. See
    /// Host Aggregates for more details.
    #[command(about = "Availability zones")]
    AvailabilityZone(Box<AvailabilityZoneArgs>),
    Extension(Box<ExtensionArgs>),
    Flavor(Box<FlavorArgs>),
    Hypervisor(Box<HypervisorArgs>),
    Keypair(Box<KeypairArgs>),
    Server(Box<ServerArgs>),
}

pub struct ComputeSrvCommand {
    pub args: ComputeSrvArgs,
}

impl OSCCommand for ComputeSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ComputeSrvCommands::Aggregate(args) => AggregateCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::AvailabilityZone(args) => AvailabilityZoneCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::Extension(args) => ExtensionCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::Flavor(args) => FlavorCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::Hypervisor(args) => HypervisorCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::Keypair(args) => KeypairCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            ComputeSrvCommands::Server(args) => ServerCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
        }
    }
}
