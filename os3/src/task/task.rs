use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,                   // 任务状态
    pub task_cx: TaskContext,                      // 任务上下文
    pub task_syscall_info: [u32; MAX_SYSCALL_NUM], // 记录系统调用次数
}

#[derive(Copy, Clone, PartialEq, Debug)]
// 任务的四种状态
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
