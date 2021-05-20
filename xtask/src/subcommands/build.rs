use clap::{
    AppSettings,
    Clap,
};
use xshell::{
    cmd,
    pushenv,
    Result,
};

use crate::{
    subcommands::Target,
    utils::try_find_path_to_terrikon_hak,
};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Build {
    #[clap(long)]
    /// Build kernel in release mode, with optimizations
    pub release: bool,
    /// The target for which the kernel will be assembled
    #[clap(arg_enum, long, default_value = "riscv64gc-unknown-none-elf")]
    pub target: Target,
}

impl Build {
    pub fn execute(&self) -> Result<()> {
        let optional_release = if self.release {
            Some("--release")
        } else {
            None
        };
        let target = self.target.to_string();
        // TODO: rewrite without hardcoding and unwraps :)
        let rust_flags = format!(
            "-C link-arg=-T{}/src/lds/virt.lds",
            try_find_path_to_terrikon_hak().unwrap().as_os_str().to_str().unwrap().to_string()
        );
        let _e = pushenv("RUSTFLAGS", rust_flags);

        cmd!("cargo build --package hak --target {target} {optional_release...}").run()
    }
}
