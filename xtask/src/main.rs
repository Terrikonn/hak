use clap::{
    AppSettings,
    Clap,
};
use xshell::Result;

mod subcommands;
mod utils;

fn main() -> Result<()> {
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
