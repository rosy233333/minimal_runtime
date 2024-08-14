//! SBI console driver, for text output

use spinlock::SpinNoIrq;

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

static WRITE_LOCK: SpinNoIrq<()> = SpinNoIrq::new(());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

#[no_mangle]
pub fn print(args: fmt::Arguments) {
    let lock = WRITE_LOCK.lock();
    Stdout.write_fmt(args).unwrap();
    drop(lock);
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
