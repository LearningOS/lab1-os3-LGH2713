// 定义系统调用幻数
const SYSCALL_WRITE: usize = 64; // 写
const SYSCALL_EXIT: usize = 93; // 退出程序
const SYSCALL_YIELD: usize = 124; // 放弃资源占用
const SYSCALL_GET_TIME: usize = 169; // 获取时间
const SYSCALL_TASK_INFO: usize = 410; // 获取任务信息

mod fs;
pub mod process;

use fs::*;
use process::*;

use crate::task::TASK_MANAGER;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let current_index = inner.current_task;
    inner.tasks[current_index].syscall_times[syscall_id] += 1;

    drop(inner);

    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
