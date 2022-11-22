const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // 使用切片类型获取缓冲中的数据
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            // 将切片类型转换为字面量
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize // 返回打印长度
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
