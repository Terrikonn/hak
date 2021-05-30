use std::path::PathBuf;

use clap::{AppSettings, Clap};
use xshell::Result;

mod subcommands;

fn main() -> Result<()> {
    let mut xtask_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    xtask_dir.pop();
    let cargo_workspace_dir = xtask_dir;
    std::env::set_current_dir(cargo_workspace_dir).expect("Cannot set current dir to cargo workspace");
    Args::parse().execute_subcommand()
}

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "remimimimi <valent.xarin@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: crate::subcommands::Command,
}

impl Args {
    pub fn execute_subcommand(&self) -> Result<()> {
        self.subcommand.execute()
    }
}
