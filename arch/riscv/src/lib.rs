#![no_std]
#![feature(asm)]
pub mod registers;

/// Memory management unit virtual addressing mode
///
/// In 64-bit mode, we're given three different modes for the MMU:
///  * 0 - The MMU is off -- no protection and no translation PA = VA
///  * 8 - This is Sv39 mode -- 39-bit virtual addresses
///  * 9 - This is Sv48 mode -- 48-bit virtual addresses
#[repr(usize)]
pub enum SatpMode {
    Off = 0,
    Sv39 = 8,
    Sv48 = 9,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
