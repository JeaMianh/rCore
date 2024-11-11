#![no_std]
#![no_main]
mod lang_items;
mod sbi;

#[macro_use]
mod console;

use console::sys_exit;
use sbi::shutdown;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    println!("Hello, RISC-V!");
    sys_exit(9);
    shutdown();

}

fn clear_bss() {
    extern "C" { // 访问外部符号
        fn sbss(); // 指向 bss 段的起始地址
        fn ebss(); // 指向 bss 段的结束地址
    }

    // 从 sbss 到 ebss 的内存区域全部清零
    let start = sbss as usize;
    let end = ebss as usize;

    (start..end).for_each(|addr| {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    })

}

