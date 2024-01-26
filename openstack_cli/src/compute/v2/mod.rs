pub mod aggregate;
pub mod availability_zone;
pub mod extension;
pub mod flavor;
pub mod hypervisor;
pub mod keypair;
pub mod server;

use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::compute::v2::aggregate::{AggregateArgs, AggregateCommand};
use crate::compute::v2::availability_zone::{AvailabilityZoneArgs, AvailabilityZoneCommand};
use crate::compute::v2::extension::{ExtensionArgs, ExtensionCommand};
use crate::compute::v2::flavor::{FlavorArgs, FlavorCommand};
use crate::compute::v2::hypervisor::{HypervisorArgs, HypervisorCommand};
use crate::compute::v2::keypair::{KeypairArgs, KeypairCommand};
use crate::compute::v2::server::{ServerArgs, ServerCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

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
    /// Lists and gets detailed availability zone information.
    ///
    /// An availability zone is created or updated by setting the
    /// availability_zone parameter in the create, update, or
    /// create or update methods of the Host Aggregates API. See
    /// Host Aggregates for more details.
    #[command(about = "Availability zones")]
    AvailabilityZone(Box<AvailabilityZoneArgs>),
    /// Creates and manages host aggregates. An aggregate assigns metadata to
    /// groups of compute nodes.
    ///
    /// Policy defaults enable only users with the administrative role to
    /// perform operations with aggregates. Cloud providers can change these
    /// permissions through policy file configuration.
    #[command(about = "Host Aggregates")]
    Aggregate(Box<AggregateArgs>),
    /// Extension commands
    #[command(about = "Extensions")]
    Extension(Box<ExtensionArgs>),
    /// Flavor commands
    ///
    /// Flavors are a way to describe the basic dimensions of a server to be
    /// created including how much cpu, ram, and disk space are allocated to a
    /// server built with this flavor.
    #[command(about = "Flavors")]
    Flavor(Box<FlavorArgs>),
    /// Lists all hypervisors, shows summary statistics for all hypervisors
    /// over all compute nodes, shows details for a hypervisor, shows the
    /// uptime for a hypervisor, lists all servers on hypervisors that match
    /// the given hypervisor_hostname_pattern or searches for hypervisors by
    /// the given hypervisor_hostname_pattern.
    #[command(about = "Hypervisors")]
    Hypervisor(Box<HypervisorArgs>),
    /// Generates, imports, and deletes SSH keys.
    #[command(about = "Keypairs")]
    Keypair(Box<KeypairArgs>),
    /// **Servers (servers)**
    ///
    /// Lists, creates, shows details for, updates, and deletes servers.
    ///
    /// **Passwords**
    ///
    /// When you create a server, you can specify a password through the
    /// optional adminPass attribute. The password must meet the complexity
    /// requirements set by your OpenStack Compute provider. The server might
    /// enter an ERROR state if the complexity requirements are not met. In
    /// this case, a client might issue a change password action to reset the
    /// server password.
    ///
    /// If you do not specify a password, the API generates and assigns a
    /// random password that it returns in the response object. This password
    /// meets the security requirements set by the compute provider. For
    /// security reasons, subsequent GET calls do not require this password.
    ///
    /// **Server metadata**
    ///
    /// You can specify custom server metadata at server launch time. The
    /// maximum size for each metadata key-value pair is 255 bytes. The compute
    /// provider determines the maximum number of key-value pairs for each
    /// server. You can query this value through the maxServerMeta absolute
    /// limit.
    #[command(about = "Servers")]
    Server(Box<ServerArgs>),
}

pub struct ComputeSrvCommand {
    pub args: ComputeSrvArgs,
}

impl ServiceCommands for ComputeSrvCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ComputeSrvCommands::Aggregate(args) => AggregateCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::AvailabilityZone(args) => AvailabilityZoneCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Extension(args) => ExtensionCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Flavor(args) => FlavorCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Hypervisor(args) => HypervisorCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Keypair(args) => KeypairCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Server(args) => ServerCommand {
                args: *args.clone(),
            }
            .get_command(session),
        }
    }
}
