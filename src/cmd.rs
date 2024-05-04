pub(crate) mod init;
pub(crate) mod default;
pub(crate) mod _clone_url;
pub(crate) mod check;
pub(crate) mod log;

use clap::Parser;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "bgit", version, author, about, long_about = None)]
#[command(bin_name = "bgit")]
pub struct BgitCli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    pub(crate) clone_url: Option<String>
}

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "bgit", version, author, about, long_about = None)]
#[command(bin_name = "bgit")]
pub enum Commands {
    /// Print commit history
    Log,

    /// Initialise bgit
    Init,

    /// Do maintenance tasks
    Check,
}

impl BgitCli {
    pub fn new() -> Self {
        Self::parse()
    }
}