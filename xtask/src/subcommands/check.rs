use clap::{
    AppSettings,
    Clap,
};
use xshell::{
    cmd,
    Result,
};

use crate::subcommands::Target;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Check {
    /// The target for which the kernel will be assembled
    #[clap(arg_enum, long, default_value = "riscv64gc-unknown-none-elf")]
    pub target: Target,
}

impl Check {
    pub fn execute(&self) -> Result<()> {
        let target = self.target.to_string();
        // TODO: rewrite without hardcoding and unwraps :)
        cmd!("cargo check --package hak --target {target}").run()
    }
}
