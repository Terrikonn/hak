use clap::{
    AppSettings,
    Clap,
};
use xshell::{
    cmd,
    Result,
};

use crate::{
    subcommands::{
        build::Build,
        Target,
    },
    utils::try_find_path_to_terrikon_hak,
};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Run {
    /// The target for which the kernel will be assembled
    #[clap(arg_enum, long, default_value = "riscv64gc-unknown-none-elf")]
    pub target: Target,
    /// Build kernel in release mode, with optimizations
    #[clap(long)]
    pub release: bool,
    /// Command to run hak.elf kernel file, usually this is
    /// qemu emulator command for choosen target
    #[clap(
        short,
        long,
        default_value = "qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -drive \
                         if=none,format=raw,file=hdd.dsk,id=foo -device virtio-blk-device,scsi=off,drive=foo \
                         -nographic -serial mon:stdio -bios none -device virtio-rng-device -device virtio-gpu-device \
                         -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device \
                         -kernel "
    )]
    pub runner: String,
}

impl Run {
    pub fn execute(&self) -> Result<()> {
        let build = Build {
            target: self.target.clone(),
            release: self.release,
        };
        build.execute()?;

        let path_to_hdd = try_find_path_to_terrikon_hak().unwrap();
        let mut path_to_kernel = try_find_path_to_terrikon_hak().unwrap();
        path_to_kernel.push("target");
        path_to_kernel.push(self.target.to_string());
        path_to_kernel.push(
            if self.release {
                "release"
            } else {
                "debug"
            },
        );
        path_to_kernel.push("hak");

        let path_to_hdd = path_to_hdd.into_os_string().into_string().unwrap();
        let path_to_kernel = path_to_kernel.into_os_string().into_string().unwrap();

        cmd!(
            "
            qemu-system-riscv64
                -machine virt
                -cpu rv64
                -smp 4
                -m 128M
                -drive if=none,format=raw,file={path_to_hdd}/hdd.dsk,id=foo
                -device virtio-blk-device,scsi=off,drive=foo
                -nographic
                -serial mon:stdio
                -bios none
                -device virtio-rng-device
                -device virtio-gpu-device
                -device virtio-net-device
                -device virtio-tablet-device
                -device virtio-keyboard-device
                -kernel {path_to_kernel}
        "
        )
        .run()

        // let runner = &self.get_default_runner();
        // cmd!("{runner}").run()
    }

    // #[rustfmt::skip]
    // pub fn get_default_runner(&self) -> String {}
}
