use clap::{AppSettings, Clap};
use xshell::{cmd, Result};

use crate::subcommands::Target;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Build {
    #[clap(long)]
    /// Build kernel in release mode, with optimizations
    pub release: bool,
    /// The target for which the kernel will be assembled
    #[clap(flatten)]
    pub target: Target,
}

impl Build {
    pub fn execute(&self) -> Result<()> {
        let target = self.target.to_string();
        let is_release = self.release.then(|| "--release");
        let do_build_std = if self.target.is_build_core_target() {
            &["-Z", "build-std=core", "-Z", "build-std-features=compiler-builtins-mem"]
        } else {
            &[][..]
        };

        cmd!("cargo build --package hak --target {target} {is_release...} {do_build_std...}").run()
    }
}
