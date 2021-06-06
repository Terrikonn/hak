#![no_main]
#![no_std]
#![feature(allocator_api, alloc_error_handler, const_raw_ptr_to_usize_cast, lang_items)]
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

use bootloader::{
    entry_point,
    BootInfo,
};
use hak::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Hello from kernel!");
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut value = 0;
        for byte in framebuffer.buffer_mut().iter_mut().step_by(2) {
            *byte = value;
            value = value.wrapping_add(1);
        }
    }

    arch::independent::low_power_loop();
}

/// Exception handler presonality
///
/// Empty function for compiler
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Custom panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);

    arch::independent::low_power_loop();
}
