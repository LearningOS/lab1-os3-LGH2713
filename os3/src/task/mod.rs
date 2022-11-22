mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UPSafeCell;
use lazy_static::*;
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    // 任务总数
    num_app: usize,
    // 使用内部值获得可变变量
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    // 任务列表
    tasks: [TaskControlBlock; MAX_APP_NUM],
    // 当前任务
    current_task: usize,
}

// 初始化任务管理器
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i, t) in tasks.iter_mut().enumerate().take(num_app) {
            t.task_cx = TaskContext::goto_restore(init_app_cx(i));
            t.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}
