#[allow(unused, dead_code)]
use std::process::exit;
use crate::cmd::{BgitCli, Commands};
use crate::cmd::_clone_url::clone;
use crate::cmd::check::check;
use crate::cmd::default::default_cmd_workflow;
use crate::cmd::init::init;
use crate::cmd::log::log;

mod rules;
mod cmd;
mod events;
mod tasks;

fn main() {
    let cli_instance = BgitCli::new();

    if cli_instance.command.is_some() && cli_instance.clone_url.is_some() {
        eprintln!("FATAL! Confusing args provided!");
        exit(1);
    }

    match cli_instance.command {
        Some(Commands::Log) => {log()}
        Some(Commands::Init) => {init()}
        Some(Commands::Check) => {check()}
        None => {
            match cli_instance.clone_url {
                Some(clone_url) => {
                    clone(&clone_url)
                }
                None => {
                    default_cmd_workflow()
                }
            }
        }
    }
}
