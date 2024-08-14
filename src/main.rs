//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
// #![feature(panic_info_message)]

extern crate alloc;

use core::arch::global_asm;
use log::*;
#[cfg(feature = "smp")]
use sbi::start_secondary_cpu;
#[cfg(feature = "smp")]
use sbi_rt::hart_get_status;
#[cfg(feature = "smp")]
use app::app_main_secondary;

use app::app_main;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod app;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
#[no_mangle]
// #[inline(never)]
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // println!("[{}, {})", sbss as usize, ebss as usize);
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
// pub fn clear_bss(sbss: usize, ebss: usize) {
//     (sbss..ebss).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
// }

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main(hartid: usize, device_tree_addr: usize) -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn heap_base(); // heap base addr
        fn heap_upper_bound(); // heap upper bound
        fn boot_stack_lower_bound_0(); // stack lower bound
        fn boot_stack_top_0(); // stack top
        fn boot_stack_lower_bound_1(); // stack lower bound
        fn boot_stack_top_1(); // stack top
        fn boot_stack_lower_bound_2(); // stack lower bound
        fn boot_stack_top_2(); // stack top
        fn boot_stack_lower_bound_3(); // stack lower bound
        fn boot_stack_top_3(); // stack top
        fn boot_stack_lower_bound_4(); // stack lower bound
        fn boot_stack_top_4(); // stack top
        fn boot_stack_lower_bound_5(); // stack lower bound
        fn boot_stack_top_5(); // stack top
        fn boot_stack_lower_bound_6(); // stack lower bound
        fn boot_stack_top_6(); // stack top
        fn boot_stack_lower_bound_7(); // stack lower bound
        fn boot_stack_top_7(); // stack top
        // fn _percpu_start();
        // fn _percpu_end();
    }

    // // 先使sp指向栈
    // let stacks_base = boot_stack_lower_bound_0 as usize;
    // let stacks_size = boot_stack_top_0 as usize - stacks_base;
    // let hart_stack_top = boot_stack_top_0 as usize + stacks_size * hartid;
    // unsafe { asm!(
    //     "mv    sp, {0}", 
    //     in(reg) hart_stack_top
    // ); }

    // clear_bss(sbss as usize, ebss as usize);
    clear_bss();
    logging::init();
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] heap base={:#x}, upper_bound={:#x}",
        heap_base as usize, heap_upper_bound as usize
    );
    warn!(
        "[kernel] boot_stack 0 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_0 as usize, boot_stack_lower_bound_0 as usize
    );
    warn!(
        "[kernel] boot_stack 1 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_1 as usize, boot_stack_lower_bound_1 as usize
    );
    warn!(
        "[kernel] boot_stack 2 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_2 as usize, boot_stack_lower_bound_2 as usize
    );
    warn!(
        "[kernel] boot_stack 3 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_3 as usize, boot_stack_lower_bound_3 as usize
    );
    warn!(
        "[kernel] boot_stack 4 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_4 as usize, boot_stack_lower_bound_4 as usize
    );
    warn!(
        "[kernel] boot_stack 5 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_5 as usize, boot_stack_lower_bound_5 as usize
    );
    warn!(
        "[kernel] boot_stack 6 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_6 as usize, boot_stack_lower_bound_6 as usize
    );
    warn!(
        "[kernel] boot_stack 7 top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top_7 as usize, boot_stack_lower_bound_7 as usize
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        sbss as usize, ebss as usize
    );
    // info!(
    //     "[kernel] .percpu [{:#x}, {:#x})",
    //     _percpu_start as usize, _percpu_end as usize
    // );

    let BoardInfo {
        smp,
        frequency,
        uart,
    } = BoardInfo::parse(device_tree_addr);

    let _unused = uart;

    info!("boot hart id: {hartid}");
    info!("smp: {smp}");
    info!("timebase frequency: {frequency}");
    info!("dtb physical address: {device_tree_addr:#x}");

    axalloc::global_init(heap_base as usize, heap_upper_bound as usize - heap_base as usize);

    // 初始化percpu库
    #[cfg(feature = "smp")]
    percpu::init(smp);

    #[cfg(feature = "smp")]
    for secondary_hart_id in 0 .. smp {
        if secondary_hart_id != hartid {
            info!("will start secondary {}, status: {:?}", secondary_hart_id, hart_get_status(secondary_hart_id));
            let result = start_secondary_cpu(secondary_hart_id);
            info!("starting secondary {}, result: {:?}", secondary_hart_id, result);
        }
    }

    // 执行main函数
    app_main(hartid, smp, frequency);

    // CI autotest success: sbi::shutdown(false)
    // CI autotest failed : sbi::shutdown(true)
    
    sbi::shutdown(false)
}

/// the entry point for secondary cpus.
#[no_mangle]
pub fn rust_main_secondary(hartid: usize) -> ! {
    // extern "C" {
    //     fn boot_stack_lower_bound_0(); // stack lower bound
    //     fn boot_stack_top_0(); // stack top
    // }

    // // 先使sp指向栈
    // let stacks_base = boot_stack_lower_bound_0 as usize;
    // let stacks_size = boot_stack_top_0 as usize - stacks_base;
    // let hart_stack_top = boot_stack_top_0 as usize + stacks_size * hartid;
    // unsafe { asm!(
    //     "mv    sp, {0}", 
    //     in(reg) hart_stack_top
    // ); }

    info!("secondary cpu {} started!", hartid);

    // 执行用户指定的、运行在副CPU上的main函数
    #[cfg(feature = "smp")]
    app_main_secondary(hartid);

    loop { }
}


struct BoardInfo {
    smp: usize,
    frequency: u64,
    uart: usize,
}

impl BoardInfo {
    fn parse(dtb_pa: usize) -> Self {
        use dtb_walker::{Dtb, DtbObj, HeaderError as E, Property, Str, WalkOperation::*};

        let mut ans = Self {
            smp: 0,
            frequency: 0,
            uart: 0,
        };
        unsafe {
            Dtb::from_raw_parts_filtered(dtb_pa as _, |e| {
                matches!(e, E::Misaligned(4) | E::LastCompVersion(_))
            })
        }
        .unwrap()
        .walk(|ctx, obj| match obj {
            DtbObj::SubNode { name } => {
                if ctx.is_root() && (name == Str::from("cpus") || name == Str::from("soc")) {
                    StepInto
                } else if ctx.name() == Str::from("cpus") && name.starts_with("cpu@") {
                    ans.smp += 1;
                    StepOver
                } else if ctx.name() == Str::from("soc")
                    && (name.starts_with("uart") || name.starts_with("serial"))
                {
                    StepInto
                } else {
                    StepOver
                }
            }
            DtbObj::Property(Property::Reg(mut reg)) => {
                if ctx.name().starts_with("uart") || ctx.name().starts_with("serial") {
                    ans.uart = reg.next().unwrap().start;
                }
                StepOut
            }
            DtbObj::Property(Property::General { name, value }) => {
                if ctx.name() == Str::from("cpus") && name == Str::from("timebase-frequency") {
                    ans.frequency = match *value {
                        [a, b, c, d] => u32::from_be_bytes([a, b, c, d]) as _,
                        [a, b, c, d, e, f, g, h] => u64::from_be_bytes([a, b, c, d, e, f, g, h]),
                        _ => unreachable!(),
                    };
                }
                StepOver
            }
            DtbObj::Property(_) => StepOver,
        });
        ans
    }
}
