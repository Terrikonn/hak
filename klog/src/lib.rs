#![no_std]
use core::fmt::Write;

use logist::{
    Level,
    LevelFilter,
    Log,
    Metadata,
    Record,
    SetLoggerError,
};
use owo_colors::{
    AnsiColors,
    DynColors,
    OwoColorize,
};
use spin::Mutex;
use uart_16550::SerialPort;

// TODO: make abstract debug printer in arch crate

lazy_static::lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    hal::interrupts::free(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
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
pub struct KernelLogger {
    /// The default logging level
    default_level: LevelFilter,
}

impl KernelLogger {
    /// Initializes the global logger with a SimpleLogger instance with
    /// default log level set to `Level::Trace`.
    ///
    /// You may use the various builder-style methods on this type to configure
    /// the logger, and you must call [`init`] in order to start logging
    /// messages.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::new().init().unwrap();
    /// logist::warn!("This is an example message.");
    /// ```
    ///
    /// [`init`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    pub const fn new() -> KernelLogger {
        KernelLogger {
            default_level: LevelFilter::Debug,
        }
    }

    /// Set the 'default' log level.
    ///
    /// You can override the default level for specific modules and their
    /// sub-modules using [`with_module_level`]
    ///
    /// [`with_module_level`]: #method.with_module_level
    #[must_use = "You must call init() to begin logging"]
    pub fn with_level(mut self, level: LevelFilter) -> KernelLogger {
        self.default_level = level;
        self
    }

    /// 'Init' the actual logger, instantiate it and configure it,
    /// this method MUST be called in order for the logger to be effective.
    pub fn init(&'static self) -> Result<(), SetLoggerError> {
        logist::set_logger(self).map(|_| logist::set_max_level(self.default_level))
    }
}

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter() <= &self.default_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level_string_color = match record.level() {
            Level::Emerg => DynColors::Ansi(AnsiColors::Magenta),
            Level::Alert => DynColors::Ansi(AnsiColors::Yellow),
            Level::Crit => DynColors::Ansi(AnsiColors::Red),
            Level::Error => DynColors::Ansi(AnsiColors::BrightRed),
            Level::Warn => DynColors::Ansi(AnsiColors::BrightYellow),
            Level::Notice => DynColors::Ansi(AnsiColors::Green),
            Level::Info => DynColors::Ansi(AnsiColors::Blue),
            Level::Debug => DynColors::Ansi(AnsiColors::Cyan),
        };
        let target = if !record.target().is_empty() {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };
        println!(
            "{:>6} [{}] {}",
            record.level().color(level_string_color),
            target,
            record.args()
        );
    }

    fn flush(&self) {}
}
