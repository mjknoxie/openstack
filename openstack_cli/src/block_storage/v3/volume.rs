use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod list;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct VolumeArgs {
    #[command(subcommand)]
    command: VolumeCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VolumeCommands {
    /// List Volumes
    List(list::VolumesArgs),
    /// Show single volume
    Show(show::VolumeArgs),
}

pub struct VolumeCommand {
    pub args: VolumeArgs,
}

impl ResourceCommands for VolumeCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            VolumeCommands::List(args) => Box::new(list::VolumesCmd { args: args.clone() }),
            VolumeCommands::Show(args) => Box::new(show::VolumeCmd { args: args.clone() }),
        }
    }
}