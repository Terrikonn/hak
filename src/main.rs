#![no_main]
#![no_std]
#![feature(
    panic_info_message,
    asm,
    global_asm,
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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Exception handler presonality
///
/// Empty function for compiler
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Custom panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
    // print!("Aborting: ");
    // if let Some(p) = info.location() {
    //     println!("line {}, file {}: {}", p.line(), p.file(), info.message().unwrap());
    // } else {
    //     println!("no information available.");
    // }
    // abort();
}

// /// Never return function that waits for interrupt
// ///
// /// Used in `panic` to handle end of kernel
// /// execution
// #[no_mangle]
// extern "C" fn abort() -> ! {
//     loop {
//         unsafe {
//             asm!("wfi");
//         }
//     }
// }
