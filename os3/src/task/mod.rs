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

// 任务控制器
pub struct TaskManager {
    // 任务总数
    num_app: usize,
    // 使用内部值获得可变变量
    inner: UPSafeCell<TaskManagerInner>,
}

// 任务控制器内部数据
pub struct TaskManagerInner {
    // 任务列表
    tasks: [TaskControlBlock; MAX_APP_NUM],
    // 当前任务
    current_task: usize,
}

// 初始化任务管理器
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app(); // 获取应用总数
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM]; // 初始化任务列表，每个元素是一个任务控制块
        for (i, t) in tasks.iter_mut()// 获取可变切片迭代器
        .enumerate()// 创建一个pair(i, val)迭代器
        .take(num_app) {
            t.task_cx = TaskContext::goto_restore(init_app_cx(i)); // 初始化任务上下文
            t.task_status = TaskStatus::Ready; // 初始化任务状态
        }
        // 返回一个任务管理器
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
