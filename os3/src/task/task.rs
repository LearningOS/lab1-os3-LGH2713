use super::TaskContext;

use crate::config::MAX_SYSCALL_NUM;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus, // 任务状态
    pub task_cx: TaskContext,    // 任务上下文
    pub syscall_times: Vec<u32>, // 记录系统调用次
    pub first_run: bool,
    pub begin_time: usize,
    pub end_time: usize,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_times: vec![0; MAX_SYSCALL_NUM],
            first_run: true,
            begin_time: 0,
            end_time: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
// 任务的四种状态
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
