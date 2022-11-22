use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
// 中断上下文
pub struct TrapContext {
    pub x: [usize; 32],   // 32个x寄存器
    pub sstatus: Sstatus, // 陷入状态
    pub sepc: usize,      // trap发生前执行的最后一条指令的地址
}

impl TrapContext {
    // 设置栈指针（x2 为栈指针寄存器）
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
