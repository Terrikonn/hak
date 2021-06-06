alias b := build
alias c := check
alias bi := build_image
alias r := run

target := "x86_64-unknown-none-elf.json"
build_core := if target == "x86_64-unknown-none-elf.json" { "-Z build-std=core -Z build-std-features=compiler-builtins-mem" } else { "" }

default: run

# Build kernel for given target
build:
	cargo b --package hak --target {{target}} {{build_core}}

# Check kernel for errors without build
check:
	cargo check --package hak --target {{target}} {{build_core}}

kernel_manifest_path := justfile_directory() + "/Cargo.toml"
target_dir := justfile_directory() + "/target/"
out_dir := target_dir + if target == "x86_64-unknown-none-elf.json" { "x86_64-unknown-none-elf"  } else { target }
kernel_binary_path := out_dir + "/debug/hak"
# Build bootable image of kernel
build_image: build
	cd {{justfile_directory()}}/../bootloader && \
	cargo builder \
		--kernel-manifest {{kernel_manifest_path}} \
		--kernel-binary {{kernel_binary_path}} \
		--target-dir {{target_dir}} \
		--out-dir {{out_dir}}


# TODO: rewrite without hardcoding
kernel_image := out_dir + "/debug/boot-bios-hak.img"
# Run kernel in qemu
run: build_image
	qemu-system-x86_64 \
	    -drive format=raw,file={{kernel_image}} \
		-serial stdio \
		-enable-kvm

# setup_disk:
# 	fallocate -l 32M hdd.dsk
# 	sudo losetup /dev/loop0 hdd.dsk
# 	sudo mkfs.minix -3 /dev/loop0
# 	sudo mount /dev/loop0 /mnt
# 	echo "Hello, this is my first file on Minix 3's filesystem" | sudo tee /mnt/hello.txt
# 	stat /mnt/hello.txt
# 	sudo sync /mnt

# unmount_disk:
# 	sudo umount /mnt
# 	sudo losetup -d /dev/loop0

# mount_disk:
# 	sudo losetup /dev/loop0 hdd.dsk
# 	sudo mount /dev/loop0 /mnt
