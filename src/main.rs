#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Print "Hello, World!" to the screen (VGA text mode)
    let vga_buffer = 0xb8000 as *mut u8;
    let hello = "Hello, World!";
    for (i, byte) in hello.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07; // Light grey text
        }
    }

    loop {}
}