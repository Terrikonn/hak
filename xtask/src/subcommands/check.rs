use clap::{AppSettings, Clap};
use xshell::{Cmd, Result};

use crate::subcommands::Target;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Check {
    /// The target for which the kernel will be assembled
    #[clap(flatten)]
    pub target: Target,
}

impl Check {
    pub fn execute(&self) -> Result<()> {
        let target = self.target.to_string();
        let do_build_std = self
            .target
            .is_build_core_target()
            .then(|| ["-Z", "build-std=core", "-Z", "build-std-features=compiler-builtins-mem"].iter());

        let mut cmd = Cmd::new("cargo").arg("check").args(["--package", "hak", "--target"].iter()).arg(target);
        if let Some(build_std) = do_build_std {
            cmd = cmd.args(build_std);
        }

        cmd.run()
    }
}
