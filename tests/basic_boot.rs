#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(void_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use void_os::println;
use void_os::vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    void_os::test_panic_handler(info)
}
