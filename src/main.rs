#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(lang_items)]
#![feature(llvm_asm)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

// for memcet memcpy memmove memcmp in baremetal
// https://docs.rs/rlibc/1.0.0/rlibc/
extern crate alloc;
extern crate rlibc;

mod dev;
mod interrupt;
mod mem;
mod shell;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use dev::uart::Uart;
use mem::KernelAllocator;

#[global_allocator]
static GLOBAL: KernelAllocator = KernelAllocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut SHELL: shell::Shell = shell::Shell { buf: [0; 256] };

#[no_mangle]
pub extern "C" fn __start_kernel() {
    // 割り込み関連初期化
    interrupt::init();

    // メモリ初期化
    mem::init();

    #[cfg(test)]
    test_main();

    // シェル起動
    #[cfg(not(test))]
    unsafe {
        Uart::get_instance().subscribe(Box::new(&mut SHELL));
        SHELL.start();
    }
}

#[cfg(test)]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}

// テストを実行できるようになったが、assertの結果が表示されない。。。
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    Uart::get_instance().send(b"test start\n");
    for test in tests {
        test();
    }
    Uart::get_instance().send(b"all test complete\n");
}
