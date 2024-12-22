#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel entry point
    let vga_buffer = 0xb8000 as *mut u8;
    let message = "Kernel Initialized!";
    
    // Output message to VGA screen
    for (i, byte) in message.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }

    loop {}
}