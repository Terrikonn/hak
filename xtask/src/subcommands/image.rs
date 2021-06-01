use std::path::PathBuf;

use clap::{
    AppSettings,
    Clap,
};
use xshell::{
    cmd,
    pushd,
    Result,
};

use crate::subcommands::{
    build::Build,
    path_to_kernel_bin,
    Target,
};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Image {
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

impl Image {
    pub fn execute(&self) -> Result<()> {
        self.run_build_step()?;
        self.build_image()?;
        Ok(())
    }

    fn build_image(&self) -> Result<()> {
        let _d = pushd("../bootloader");

        let path_to_kernel_bin = PathBuf::from(format!("../hak/{}", path_to_kernel_bin(&self.target, self.release)));
        let kernel_manifest_path = "../hak/Cargo.toml";
        let kernel_binary_path = path_to_kernel_bin.as_os_str();
        let target_dir = "../hak/target";
        let out_dir = path_to_kernel_bin.parent().unwrap().as_os_str();

        cmd!(
            "
            cargo builder
                --kernel-manifest {kernel_manifest_path}
                --kernel-binary {kernel_binary_path}
                --target-dir {target_dir}
                --out-dir {out_dir}
        "
        )
        .run()?;

        Ok(())
    }

    fn run_build_step(&self) -> Result<()> {
        // TODO: impl From<Image> for Build
        Build {
            target: self.target.clone(),
            release: self.release,
        }
        .execute()
    }
}

#[derive(Clap, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum FirmwareStandard {
    Bios,
    Uefi,
}
