use std::{fmt, ops::Deref};

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
pub struct Target {
    #[clap(arg_enum, long, default_value = "x86_64-unknown-none-elf")]
    target: TargetType,
}

impl Deref for Target {
    type Target = TargetType;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

#[derive(Clap, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum TargetType {
    Riscv64gcUnknownNoneElf,
    #[clap(name = "x86_64-unknown-none-elf")]
    X86_64UnknownNoneElf,
}

impl TargetType {
    /// Returns `true` if custom json target.
    pub fn is_build_core_target(&self) -> bool {
        matches!(self, Self::X86_64UnknownNoneElf)
    }
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Riscv64gcUnknownNoneElf => write!(f, "riscv64gc-unknown-none-elf"),
            Self::X86_64UnknownNoneElf => {
                write!(f, "x86_64-unknown-none-elf.json")
            }
        }
    }
}
