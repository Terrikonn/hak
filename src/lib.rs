#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, no_main)]
#![feature(allocator_api, alloc_error_handler)]
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

pub mod serial;
