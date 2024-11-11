fn sbicall(id: usize, args: [usize; 3]) -> isize {
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

const SBI_SHUTDOWN: usize = 8; // SBI 关机调用号

pub fn shutdown() -> !{ //  ! 类型表示函数不会返回
    sbicall(SBI_SHUTDOWN, [0, 0, 0]);
    loop {
        unsafe {
            core::arch::asm!("wfi"); // wfi 汇编指令使 CPU 进入等待状态
        }
    }
}