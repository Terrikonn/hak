use std::path::PathBuf;

use clap::{AppSettings, Clap};
use xshell::{cmd, Result};

use crate::subcommands::{build::Build, Target};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Run {
    /// The target for which the kernel will be assembled
    #[clap(flatten)]
    pub target: Target,
    /// Build kernel in release mode, with optimizations
    #[clap(long)]
    pub release: bool,
    /// Command to run kernel
    #[clap(arg_enum, short, long, default_value = "qemu")]
    pub runner_type: RunnerType,
}

impl Run {
    pub fn execute(&self) -> Result<()> {
        Build {
            target: self.target.clone(),
            release: self.release,
        }
        .execute()?;
        let path_to_kernel = path_to_kernel(&self);

        let Runner {
            command,
            args,
        } = self.target.runner(&self.runner_type);

        cmd!("{command} {args...} {path_to_kernel}").run()
    }
}

fn path_to_kernel(args: &Run) -> String {
    let mut path_to_kernel = PathBuf::from("target");
    path_to_kernel.push(args.target.to_string().chars().take_while(|c| *c != '.').collect::<String>());
    path_to_kernel.push(if args.release {
        "release"
    } else {
        "debug"
    });
    path_to_kernel.push("hak");

    path_to_kernel.into_os_string().into_string().unwrap()
}

impl super::TargetType {
    fn runner(&self, runner: &crate::subcommands::run::RunnerType) -> Runner {
        match runner {
            RunnerType::Qemu => match self {
                Self::Riscv64gcUnknownNoneElf => {
                    Runner::builder().command("qemu-system-riscv64").args(&["-kernel"]).build()
                }
                Self::X86_64UnknownNoneElf => {
                    Runner::builder().command("qemu-system-x86_64").args(&["-kernel"]).build()
                }
            },
        }
    }
}

#[non_exhaustive]
#[derive(Clap, Debug, Clone)]
pub enum RunnerType {
    #[clap(name = "qemu")]
    Qemu,
}

pub struct Runner {
    command: &'static str,
    args: Vec<&'static str>,
}

impl Runner {
    pub fn builder() -> RunnerBuilder {
        RunnerBuilder::default()
    }
}

#[derive(Default)]
pub struct RunnerBuilder {
    command: &'static str,
    args: Vec<&'static str>,
}

impl RunnerBuilder {
    pub fn command(mut self, command: &'static str) -> Self {
        self.command = command;
        self
    }

    #[allow(dead_code)]
    pub fn arg(mut self, arg: &'static str) -> Self {
        self.args.push(arg);
        self
    }

    pub fn args(mut self, args: &'static [&str]) -> Self {
        self.args.append(&mut args.to_vec());
        self
    }

    pub fn build(self) -> Runner {
        Runner {
            command: self.command,
            args: self.args,
        }
    }
}
