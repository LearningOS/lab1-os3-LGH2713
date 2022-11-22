mod context;

use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

// 加载汇编代码
core::arch::global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        // 保存陷入时的寄存器状态
        fn __alltraps();
    }
    unsafe {
        // 写入返回地址
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

// 设置时间中断使能
pub fn enable_timer_interrupt() {
    unsafe {
        // 设置监督模式下的时间中断使能
        sie::set_stimer();
    }
}

#[no_mangle] // 使用其他语言的FFI时需要添加这个宏
             // 陷入操作函数
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // 获取trap发生前处于哪个特权级等信息
    let stval = stval::read(); // 获取trap的附加信息
    match scause.cause() {
        // 异常原因：用户系统调用
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        // 异常原因：储存错误
        Trap::Exception(Exception::StoreFault) => {
            error!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, core dumped.", stval, cx.sepc);
            exit_current_and_run_next();
        }
        // 异常原因：非法指令
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            exit_current_and_run_next();
        }
        // 异常原因：监督模式时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
