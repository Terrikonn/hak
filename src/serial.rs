use core::{
    cell::RefCell,
    lazy::Lazy,
};

use spin::Mutex;
use uart_16550::SerialPort;

// TODO: make abstract debug printer in arch crate
// NOTE: can be replaced with once_cell crate
pub static SERIAL1: Mutex<Lazy<RefCell<SerialPort>>> = Mutex::new(Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    RefCell::new(serial_port)
}));

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    use arch::independent::interrupts;

    interrupts::free(|| {
        SERIAL1.lock().borrow_mut().write_fmt(args).expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}
