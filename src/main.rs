#![feature(lang_items)]
#![feature(global_asm)]

#![no_std]
#![no_main]

use core::ptr;

global_asm!(include_str!("boot.s"));

extern "C" {
	fn system_off();
}

#[no_mangle]
pub extern "C" fn main() {
	let target = 0x09000000 as *mut char;
	unsafe {
		ptr::write_volatile(target, 'R');
	}

	unsafe { system_off(); }
}


#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() { loop {} }

#[lang = "panic_fmt"] #[no_mangle] 
pub extern fn panic_fmt() -> ! { loop {} }