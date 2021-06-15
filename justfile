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
target_triplet := if target == "x86_64-unknown-none-elf.json" { "x86_64-unknown-none-elf" } else { target }
out_dir := target_dir + target_triplet + "/debug/"
kernel_binary_path := out_dir + "hak"
# Build bootable image of kernel
build_image firmware="bios": build
	cd {{justfile_directory()}}/../bootloader && \
	cargo builder \
		--kernel-manifest {{kernel_manifest_path}} \
		--kernel-binary {{kernel_binary_path}} \
		--target-dir {{target_dir}} \
		--out-dir {{out_dir}} \
		--firmware {{firmware}}


# TODO: rewrite without hardcoding
kernel_bios_image := out_dir + "boot-bios-hak.img"
# Run kernel in qemu
run: build_image
	qemu-system-x86_64 \
	    -drive format=raw,file={{kernel_bios_image}} \
		-serial stdio \
		-enable-kvm
