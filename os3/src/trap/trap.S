.altmacro
.macro SAVE_GP n 
    sd x\n, \n*8(sp)
.endm 
.macro LOAD_GP n 
    ld x\n, \n*8(sp)
.endm
    .section .text          # 代码段
    .globl __alltraps       # 全局标识符
    .globl __restore        # 全局标识符
    .align 2                # 按照 2^2 字节对齐
__alltraps:                 # 处理陷入时上下文的保存
    csrrw sp, sscratch, sp  # 使sp指向内核栈，sscratch指向用户栈，即交换sp和sscratch的指向
    addi sp, sp, -34*8      # 在内核栈中分配一个陷入上下文大小的空间（分配34*8字节的栈帧）
    # 保存寄存器，跳过x0（硬布线为0，忽略写）, x2（栈指针）, x4（线程指针）
    sd x1, 1*8(sp)          # 保存返回地址
    sd x3, 3*8(sp)          # 保存全局指针
    .set n, 5           
    .rept 27                # 保存其余27个指针
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus        # 将sstatus的值读到临时寄存器t0
    csrr t1, sepc           # 将sepc的值读到临时寄存器t1
    sd t0, 32*8(sp)         # 将t0的值保存到对应的内核栈上
    sd t1, 33*8(sp)         # 将t1的值保存到对应的内核栈上
    csrr t2, sscratch       # 将sscratch的值读到临时寄存器t2中
    sd t2, 2*8(sp)          # 将t2的值保存到对应的内核栈上
    mv a0, sp               # 让a0指向内核栈的栈指针
    call trap_handler       # 调用陷入处理函数

__restore:                  # 处理上下文的恢复
    ld t0, 32*8(sp)         # t0栈中的数据
    ld t1, 33*8(sp)         # t1读出栈中的数据
    ld t2, 2*8(sp)          # t2读出栈中的数据
    csrw sstatus, t0        # 将t0的值写到sstatus
    csrw sepc, t1           # 将t1的值写到sepc
    csrw sscratch, t2       # 将t2的值写到sscratch
    ld x1, 1*8(sp)          # 加载返回地址
    ld x3, 3*8(sp)          # 加载全局指针
    .set n, 5       
    .rept 27                # 加载数据到其余27个寄存器
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8       # 释放栈空间
    csrrw sp, sscratch, sp  # 交换sp和sscratch的指向，sscratch指向内核栈，sp指向用户栈
    sret