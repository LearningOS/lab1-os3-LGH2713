#[derive(Copy, Clone)]
#[repr(C)]

pub struct TaskContext {
    ra: usize,      // 函数返回地址
    sp: usize,      // 内核栈指针
    s: [usize; 12], // 被调用者寄存器s0~s11
}

impl TaskContext {
    // 用0初始化任务上下文
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
