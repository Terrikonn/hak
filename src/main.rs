#![no_main]
#![no_std]
#![feature(
    panic_info_message,
    asm,
    global_asm,
    allocator_api,
    alloc_error_handler,
    alloc_prelude,
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

// #[macro_use]
extern crate alloc;
// This is experimental and requires alloc_prelude as a feature
// use alloc::prelude::v1::*;

/// Exception handler presonality
///
/// Empty function for compiler
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Custom panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_print!("Aborting: ");
    if let Some(p) = info.location() {
        serial_println!("line {}, file {}: {}", p.line(), p.file(), info.message().unwrap());
    } else {
        serial_println!("no information available.");
    }
    abort();
}

/// Never return function that waits for interrupt
///
/// Used in `panic` to handle end of kernel
/// execution
#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

extern "C" {
    fn switch_to_user(frame: usize) -> !;
}

/// Load a frame.
///
/// Since it will jump to another program counter,
/// it will never return back here. We don't care if we leak
/// the stack, since we will recapture the stack during `m_trap`.
fn rust_switch_to_user(frame: usize) -> ! {
    unsafe {
        switch_to_user(frame);
    }
}

/// Kernel entry point
#[no_mangle]
extern "C" fn kinit() {
    page::init();
    kmem::init();
    serial_println!("Hello from new uart crate");
    process::init();
    // We lower the threshold wall so our interrupts can jump over it.
    // Any priority > 0 will be able to be "heard"
    plic::set_threshold(0);
    // VIRTIO = [1..8]
    // UART0 = 10
    // PCIE = [32..35]
    // Enable PLIC interrupts.
    for i in 1..=10 {
        plic::enable(i);
        plic::set_priority(i, 1);
    }
    // Set up virtio. This requires a working heap and page-grained allocator.
    virtio::probe();
    // Test the block driver!
    process::add_kernel_process(test::test);

    // Get the GPU going
    virtio::gpu::init(6);
    // We schedule the next context switch using a multiplier of 1
    // Block testing code removed.
    trap::schedule_next_context_switch(1);
    rust_switch_to_user(sched::schedule());
    // switch_to_user will not return, so we should never get here
}

/// Function for hardware thread(hart) initialization
#[no_mangle]
extern "C" fn kinit_hart(_hartid: usize) {
    // We aren't going to do anything here until we get SMP going.
    // All non-0 harts initialize here.
}

/// Export RISC-V assembly files for bootloader and trap handler
pub mod assembly;
/// Buffer management stuff
pub mod buffer;
/// RISC-V cpu instructions wrapper
pub mod cpu;
/// Elf binary format execution
pub mod elf;
/// Minix3 file system implementation
pub mod fs;
/// Kernel memory management
pub mod kmem;
/// Synchronization primitives
pub mod lock;
/// Paging and related functions implementation
pub mod page;
/// Programmable interrupt controller functionality
pub mod plic;
/// Process data
pub mod process;
/// Process scheduling
pub mod sched;
/// Output through serial port
pub mod serial;
/// System calls
pub mod syscall;
/// First initalized process
pub mod test;
/// Trampoline for interrupts
pub mod trap;
/// Virtual input/output protocol
pub mod virtio;
