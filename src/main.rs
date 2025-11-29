#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static pointer to a slice of bytes
static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Hello World
    vga_buffer::print_test();

    loop {}
}
