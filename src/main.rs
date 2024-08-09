#![allow(unused_attributes, dead_code)]
use crate::cmd::check::check;
use crate::cmd::default::default_cmd_workflow;
use crate::cmd::init::init;
use crate::cmd::log::log;
use crate::cmd::{Cli, Commands};

mod bgit_error;
mod cmd;
mod events;
mod rules;
mod step;
mod workflows;
mod workflow_queue;

fn main() {
    let cli_instance_wrap = Cli::new();

    if let Some(cli_instance) = cli_instance_wrap {
        match cli_instance.command {
            Some(Commands::Log) => log(),
            Some(Commands::Init) => init(),
            Some(Commands::Check) => check(),
            None => default_cmd_workflow(),
        }
    }
}
