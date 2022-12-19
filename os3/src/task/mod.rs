mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::config::{CLOCK_FREQ, MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UPSafeCell;
use crate::timer::{get_time, get_time_ms};
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::*;
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

// 任务控制器
pub struct TaskManager {
    // 任务总数
    num_app: usize,
    // 使用内部值获得可变变量
    pub inner: UPSafeCell<TaskManagerInner>,
}

// 任务控制器内部数据
pub struct TaskManagerInner {
    // 任务列表
    pub tasks: Vec<TaskControlBlock>,
    // 当前任务
    pub current_task: usize,
}

// 初始化任务管理器
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app(); // 获取应用总数
        let mut  tasks: Vec<TaskControlBlock>  =  vec![TaskControlBlock::new(); MAX_APP_NUM]; // 初始化任务列表，每个元素是一个任务控制块
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

impl TaskManager {
    fn count_syscall(&self, syscall_id: usize) {
        if syscall_id < MAX_SYSCALL_NUM {
            let mut inner = TASK_MANAGER.inner.exclusive_access();
            let current_task = inner.current_task;
            inner.tasks[current_task].syscall_times[syscall_id] += 1;
        }
    }
    fn get_current_task_status(&self) -> TaskStatus {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].task_status
    }
    fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        let inner = TASK_MANAGER.inner.exclusive_access();
        *inner.tasks[inner.current_task].syscall_times
    }
    fn get_current_run_time(&self) -> usize {
        let inner = TASK_MANAGER.inner.exclusive_access();
        get_time_ms() - inner.tasks[inner.current_task].start_time.unwrap()
    }
    // 运行第一个任务
    fn run_first_task(&self) -> ! {
        // 获取数据的可变借用
        let mut inner = self.inner.exclusive_access();
        // 获取任务列表的第一个任务的可变借用
        let task0 = &mut inner.tasks[0];
        // 设置第一个任务为运行状态
        task0.task_status = TaskStatus::Running;
        task0.start_time = Some(get_time_ms());
        //
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        // 清理变量inner
        drop(inner);
        // 初始化一个空任务
        let mut _unused = TaskContext::zero_init();
        unsafe {
            // 切换当前任务和空任务
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    // 改变当前任务状态 Running => Ready
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task; // 获取当前任务索引
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    // 改变当前任务状态 Running => Exited
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    // 找到下一个任务然后返回任务id
    // 只返回任务列表中第一个状态为Ready的任务
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    // 执行下一个任务
    fn run_next_task(&self) {
        // 寻找下一个任务
        if let Some(next) = self.find_next_task() {
            // 记录当前任务的索引
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            // 将下一个任务的状态置为Running
            inner.tasks[next].task_status = TaskStatus::Running;
            // 将当前任务置为下一个任务
            inner.current_task = next;
            // 获取当前任务上下文指针
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            // 获取下一个任务上下文指针
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            if inner.tasks[next].start_time.is_none() {
                inner.tasks[next].start_time = Some(get_time_ms());
            }
            // 释放inner
            drop(inner);
            unsafe {
                // 交换当前任务和下一个任务的指针指向
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All application completed!");
        }
    }
}

// 执行第一个任务
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

// 执行下一个任务
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

// Running => Ready
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

// Running => Exited
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

// 暂停当前任务然后执行任务列表中的下一个任务
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

// 退出当前任务然后执行任务列表的下一个任务
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn count_syscall(syscall_id: usize) {
    TASK_MANAGER.count_syscall(syscall_id);
}

pub fn get_current_task_status() -> TaskStatus {
    TASK_MANAGER.get_current_task_status()
}

pub fn get_syscall_times() -> [u32; MAX_SYSCALL_NUM] {
    TASK_MANAGER.get_syscall_times()
}

pub fn get_current_run_time() -> usize {
    TASK_MANAGER.get_current_run_time()
}
