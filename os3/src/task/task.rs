use super::TaskContext;

use crate::config::MAX_SYSCALL_NUM;
use alloc::boxed::Box;

#[derive(Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,                    // 任务状态
    pub task_cx: TaskContext,                       // 任务上下文
    pub syscall_times: Box<[u32; MAX_SYSCALL_NUM]>, // 记录系统调用次
    pub start_time: Option<usize>,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_times: Box::new([0; MAX_SYSCALL_NUM]),
            start_time: None,
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
