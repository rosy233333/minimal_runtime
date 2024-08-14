use core::arch::asm;

use lazy_init::LazyInit;
use log::warn;
use riscv::register::scause::{Exception, Trap};
use spinlock::SpinNoIrqOnly;
use task_management::*;
use trap_handler::*;

static TIMEBASE_FREQUENCY: LazyInit<u64> = LazyInit::new();

struct CurrentTimebaseFrequencyImpl;

#[crate_interface::impl_interface]
impl CurrentTimebaseFrequency for CurrentTimebaseFrequencyImpl {
    // 获取dtb上的时基频率比较困难，因此乱填的
    fn current_timebase_frequency() -> usize {
        *TIMEBASE_FREQUENCY as usize
    }
}

#[allow(dead_code)]
pub(crate) fn app_main(cpu_id: usize, cpu_num: usize, timebase_frequency: u64) {
    TIMEBASE_FREQUENCY.init_by(timebase_frequency);
    task_management::init_main_processor(cpu_id, cpu_num);
    trap_handler::init_main_processor();
    start_main_processor(test_block_wake_yield);
    // start_main_processor(test_interrupt);
    // start_main_processor(test_preempt);
}

#[cfg(feature = "smp")]
#[allow(dead_code)]
pub(crate) fn app_main_secondary(cpu_id: usize) {
    task_management::init_secondary_processor(cpu_id);
    trap_handler::init_secondary_processor();
    start_secondary_processor();
}

static BLOCK_QUEUE: LazyInit<SpinNoIrqOnly<BlockQueue>> = LazyInit::new();

#[no_mangle]
#[allow(dead_code)]
fn test_block_wake_yield() -> i32 {

    BLOCK_QUEUE.init_by(SpinNoIrqOnly::new(BlockQueue::new()));
    for _i in 0 .. 2 {
        spawn_to_global(|| {
            let task_id = current_id();
            warn!("task {task_id} is thread");
            let cpu_id_1 = current_processor_id();
            warn!("before yield, thread {task_id} is running on cpu {cpu_id_1}");
    
            yield_current_to_local();

            let cpu_id_2 = current_processor_id();
            warn!("after yield, before block, thread {task_id} is running on cpu {cpu_id_2}");

            BlockQueue::block_current_with_locked_self(&*BLOCK_QUEUE, SpinNoIrqOnly::lock);

            let cpu_id_3 = current_processor_id();
            warn!("after wake, thread {task_id} is running on cpu {cpu_id_3}");

            exit_current(0);
            -1
        });
    }

    for _i in 0 .. 2 {
        spawn_to_global_async(async {
            let task_id = current_id();
            warn!("task {task_id} is coroutine");
            let cpu_id_1 = current_processor_id();
            warn!("before yield, coroutine {task_id} is running on cpu {cpu_id_1}");
    
            yield_current_to_local_async().await;
            warn!("async task can yield with sync method!");
            yield_current_to_local();

            let cpu_id_2 = current_processor_id();
            warn!("after yield, before block, coroutine {task_id} is running on cpu {cpu_id_2}");

            BlockQueue::block_current_async_with_locked_self(&*BLOCK_QUEUE, SpinNoIrqOnly::lock).await;
            warn!("async task can block with sync method!");
            BlockQueue::block_current_with_locked_self(&*BLOCK_QUEUE, SpinNoIrqOnly::lock);

            let cpu_id_3 = current_processor_id();
            warn!("after wake, coroutine {task_id} is running on cpu {cpu_id_3}");

            exit_current_async(0).await;
            -1
        });
    }

    assert!(change_current_priority(2).is_ok()); // 使主任务可以被子任务抢占
    
    loop {
        BLOCK_QUEUE.lock().wake_all_to_global();
    }
}

#[allow(dead_code)]
#[no_mangle]
fn test_interrupt() -> i32{
    register_trap_handler(Trap::Exception(Exception::Breakpoint), |_stval, task_context| {
        task_context.step_sepc(); // 使保存的sepc前进一条指令
        warn!("handle breakpoint exception!");
    });

    unsafe { asm!(
        "ebreak", // 触发Breakpoint异常（0x3）
        "
        li      t0, 0
        addi    t0, t0, -1
        sw      a0, 0(t0)
        ", // 触发Store page fault异常（0xf）
    ); }
    0
}

#[allow(dead_code)]
fn test_preempt() -> i32{
    enable_irqs();
    // main任务的初始优先级为1
    assert!(spawn_to_local_with_priority(|| {
        println!("Main task is preempted by sync task!");
        assert!(change_current_priority(2).is_ok());
        loop { }
    }, 0).is_ok());
    assert!(spawn_to_local_async_with_priority(async {
        println!("Main task is preempted by async task!");
        assert!(change_current_priority(2).is_ok());
        loop { }
    }, 0).is_ok());
    println!("task spawn complete!");
    loop { }
}