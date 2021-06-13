#![no_main]
#![cfg_attr(not(test), no_std)]
#![feature(
    allocator_api,
    alloc_error_handler,
    const_raw_ptr_to_usize_cast,
    lang_items
)]
#![warn(
    clippy::correctness,
    clippy::pedantic,
    clippy::style,
    clippy::restriction,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

#[macro_use]
extern crate logist;

use bootloader::{
    entry_point,
    BootInfo,
};

entry_point!(kernel_main);

static LOGGER: &'static klog::KernelLogger = &klog::KernelLogger::new();

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    LOGGER.init().unwrap();
    debug!("This is an example message.");

    arch::independent::init();

    arch::independent::low_power_loop();
}

/// Custom panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    klog::println!("{}", info);

    arch::independent::low_power_loop();
}
