#![allow(unused)]

use sbi_rt::{hart_get_status, hart_stop, hart_suspend, SbiRet};

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/// use sbi call to getchar from console (qemu uart handler)
pub fn console_getchar() -> usize {
    #[allow(deprecated)]
    sbi_rt::legacy::console_getchar()
}

/// use sbi call to shutdown the kernel
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

/// start secondary cpu
pub fn start_secondary_cpu(hartid: usize) -> SbiRet {
    use sbi_rt::hart_start;
    extern "C" {
        fn _start_secondary(hartid: usize);
    }
    hart_start(hartid, _start_secondary as usize, 0)
}