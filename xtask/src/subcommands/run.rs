use std::path::Path;

use clap::{
    AppSettings,
    Clap,
};
use xshell::{
    cmd,
    Result,
};

use crate::subcommands::{
    image::{
        FirmwareStandard,
        Image,
    },
    path_to_kernel_bin,
    Target,
};

// TODO: Rewrite separated runners for bochs and qemu as `run` subcommands
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Run {
    /// The target for which the kernel will be assembled
    #[clap(flatten)]
    pub target: Target,
    /// Build kernel in release mode, with optimizations
    #[clap(long)]
    pub release: bool,
    /// Firmware standard for bootloader
    #[clap(arg_enum, long, default_value = "bios")]
    pub firmware_standard: FirmwareStandard,
}

impl Run {
    pub fn execute(&self) -> Result<()> {
        let image_step = Image {
            target: self.target.clone(),
            release: self.release,
            firmware_standard: self.firmware_standard.clone(),
        };
        image_step.execute()?;

        let path_to_kernel_bin = path_to_kernel_bin(&self.target, self.release);

        let Runner {
            command,
            args,
        } = self.target.runner(path_to_kernel_bin.parent().unwrap());

        cmd!("{command} {args...}").run()
    }
}

impl super::TargetType {
    // TODO: Move runner function to `run` emulators subcommands eg
    // `cargo xtask run qemu`
    // `cargo xtask run bochs`
    // Also add `--` like in cargo to pass extra arguments
    // Plus add uefi|bios branching there, currently only bios
    fn runner(&self, path_to_out_dir: &Path) -> Runner {
        match self {
            Self::Riscv64gcUnknownNoneElf => unimplemented!("riscv currently unsupported"),
            Self::X86_64UnknownNoneElf => Runner::builder()
                .command("qemu-system-x86_64")
                .arg("-drive")
                .arg(&format!(
                    "format=raw,file={}",
                    path_to_out_dir.join("boot-bios-hak.img").into_os_string().to_str().unwrap()
                ))
                .arg("-serial")
                .arg("stdio")
                .build(),
        }
    }
}

pub struct Runner {
    command: String,
    args: Vec<String>,
}

impl Runner {
    pub fn builder() -> RunnerBuilder {
        RunnerBuilder::default()
    }
}

#[derive(Default)]
pub struct RunnerBuilder {
    command: String,
    args: Vec<String>,
}

impl RunnerBuilder {
    pub fn command(mut self, command: &str) -> Self {
        self.command = command.to_string();
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn opt_arg(self, arg: Option<&str>) -> Self {
        if let Some(argument) = arg {
            self.arg(argument)
        } else {
            self
        }
    }

    pub fn build(self) -> Runner {
        Runner {
            command: self.command,
            args: self.args,
        }
    }
}
