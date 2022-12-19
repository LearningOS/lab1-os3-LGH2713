use crate::config::MAX_SYSCALL_NUM;
use crate::syscall::*;
use alloc::vec;
use alloc::vec::Vec;

use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER,
};
use crate::timer::{get_time, get_time_us};

#[repr(C)] // 和C语言规则保持一致，包括数据顺序、大小和对齐方式
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: Vec<u32>, // 系统调用次数
    pub time: usize,
}

impl TaskInfo {
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: vec![0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

// 系统调用，退出程序
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

// 当前任务放弃所有资源，交个下个任务执行
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us / 1_000_000,
        };
    }
    0
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let inner = TASK_MANAGER.inner.exclusive_access();
    let current_index = inner.current_task;
    unsafe {
        *ti = TaskInfo {
            syscall_times: inner.tasks[current_index].syscall_times.clone(),
            time: get_time() - inner.tasks[current_index].begin_time,
            status: inner.tasks[current_index].task_status,
        };
    }
    drop(inner);
    0
}
