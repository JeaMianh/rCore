fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;

    unsafe {
        core::arch::asm!(
            "ecall", // ecall 汇编指令触发系统调用
            inlateout("x10") args[0] => ret, // x10（a0）先传入参数，再传出返回值。
            in("x11") args[1], // x11（a1）传入参数
            in("x12") args[2], // x12（a2）传入参数
            in("x17") id, // x17（a7）传入系统调用号
        );
    }

    ret
}

const SYSCALL_EXIT: usize = 93; // 退出系统调用号

pub fn sys_exit(xstate: isize) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

const SYSCALL_WRITE: usize = 64; // 写系统调用号

fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

use core::fmt::Write;
struct Stdout; // 空结构体

// 实现 Write trait
impl Write for Stdout {

    // 实现 write_str 方法，将字符串写入标准输出
    // 返回值等价于 Result<(), core::fmt::Error>
    fn write_str(&mut self, s: &str) -> core::fmt::Result { 
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

// 参数由 print！一类的格式化宏生成
pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap(); // unwrap() 用于处理 Result 类型的返回值
}

// 基于 print 函数实现格式化宏
#[macro_export]
macro_rules! print { // 这啥语法啊看不懂
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}