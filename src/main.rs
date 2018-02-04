#![feature(lang_items)]
#![feature(global_asm)]

#![no_std]
#![no_main]

global_asm!(include_str!("boot.s"));


#[lang = "eh_personality"] extern fn eh_personality() {
	loop {}
}

#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt() -> ! {
	loop {}
}