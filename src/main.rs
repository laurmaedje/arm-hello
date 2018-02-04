#![feature(lang_items)]
#![feature(global_asm)]
#![feature(ptr_internals)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod serial;
mod error;


/// Startup assembly
global_asm!(include_str!("boot.s"));

/// Kernel entry point
#[no_mangle]
pub extern "C" fn main() {
    println!("Hello World!");
}
