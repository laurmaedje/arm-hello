#![allow(private_no_mangle_fns)]


/// Gets called for each stack frame if an exception occurs
#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn eh_personality() {}

/// Formats the message if a panic occurs. Never returns.
#[lang = "panic_fmt"]
#[no_mangle]
extern "C" fn rust_begin_panic(fmt: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("error: kernel panic in {} at line {}", file, line);
    println!("> {}", fmt);
    loop {}
}

#[no_mangle]
extern fn _Unwind_Resume() { loop {} }