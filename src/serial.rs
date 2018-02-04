use core::ptr;
use core::ptr::Unique;
use core::fmt;
use spin::Mutex;


const UART0: *mut u8 = 0x09000000 as *mut u8;
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    target: unsafe { Unique::new_unchecked(UART0) },
});


/// Abstracts around the [`PrimeCell UART (PL011)`][UART-Spec] and implements `fmt::Write`
/// [UART-Spec]: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0183f/DDI0183.pdf
pub struct Writer {
    target: Unique<u8>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            ptr::write_volatile(self.target.as_ptr(), byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

/// Defines the `print!` macro known from the standard library
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::serial::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

/// Defines the `println!` macro known from the standard library
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
