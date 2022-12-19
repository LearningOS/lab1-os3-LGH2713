core::arch::global_asm!(include_str!("switch.S"));

use super::TaskContext;

extern "C" {
    // 保存当前任务指针，同时切换到下一个任务指针
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
