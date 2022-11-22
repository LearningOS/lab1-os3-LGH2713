# a0 => 参数0或返回值0
# a1 => 参数1或返回值1

.altmacro
.macro SAVE_SN n # n为参数，以下的宏将“\n”全部替换为n
    sd s\n, (\n+2)*8(a0) # 例如，当n=1时,改汇编语句会被替换为 sd s1 (1+2)*8(a0)
.endm
.macro LOAD_SN n 
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # __switch(
    #    current_task_cx_ptr: *mut TaskContext,
    #    next_task_cx_ptr: *const TaskContext 
    # )
    # 保存内核栈指针到寄存器a0
    sd sp, 8(a0)
    # 保存当前任务返回地址
    sd ra, 0(a0)
    # 保存当前任务被调用者寄存器
    .set n, 0 # 定义变量n，同时赋值为0
    .rept 12  # 重复12次
        SAVE_SN %n # 依次存入每个寄存器
        .set n, n + 1
    .endr 
    # 加载下一个任务的返回地址
    ld ra, 0(a1)
    # 加载下一个任务的被调用者寄存器的数据
    .set n, 0
    .rept 12
        LOAD_SN %n # 依次加载到每个寄存器
        .set n, n + 1
    .endr
    # 恢复下个任务的内核栈
    ld sp, 8(a1)
    ret