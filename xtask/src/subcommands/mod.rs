use std::{fmt, ops::Deref, path::PathBuf};

use clap::Clap;
use xshell::Result;

pub mod build;
pub mod check;
pub mod image;
pub mod run;

#[derive(Clap, Debug)]
pub enum Command {
    /// Check kernel for compilation errors
    #[clap(alias = "c")]
    Check(check::Check),
    /// Build kernel
    #[clap(alias = "b")]
    Build(build::Build),
    /// Create bootable image of os
    #[clap(alias = "i")]
    Image(image::Image),
    /// Run kernel with emulator
    #[clap(alias = "r")]
    Run(run::Run),
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Check(check) => check.execute(),
            Self::Build(build) => build.execute(),
            Self::Image(image) => image.execute(),
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

fn path_to_kernel_bin(target: &TargetType, is_release: bool) -> String {
    let mut path_to_kernel = PathBuf::from("target");
    path_to_kernel.push(target.to_string().chars().take_while(|c| *c != '.').collect::<String>());
    path_to_kernel.push(if is_release {
        "release"
    } else {
        "debug"
    });
    path_to_kernel.push("hak");

    path_to_kernel.into_os_string().into_string().unwrap()
}
