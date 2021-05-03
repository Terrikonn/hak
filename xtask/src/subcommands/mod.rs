use std::fmt;

use clap::Clap;
use xshell::Result;

pub mod build;
pub mod check;
pub mod run;

#[derive(Clap, Debug)]
pub enum Command {
    /// Check kernel for compilation errors
    #[clap(alias = "c")]
    Check(check::Check),
    /// Build kernel
    #[clap(alias = "b")]
    Build(build::Build),
    // TOOD: Rewrite runner
    // NOTE: add subcommands to choose emulator and make emulator command builder
    /// Run kernel with emulator
    #[clap(alias = "r")]
    Run(run::Run),
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Check(check) => check.execute(),
            Self::Build(build) => build.execute(),
            Self::Run(run) => run.execute(),
        }
    }
}

#[derive(Clap, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Target {
    Riscv64gcUnknownNoneElf,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Riscv64gcUnknownNoneElf => write!(f, "riscv64gc-unknown-none-elf"),
        }
    }
}
