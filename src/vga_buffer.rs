// VGA Driver
const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

pub fn write_char(c: char) {
    unsafe {
        *VGA_BUFFER.offset(0) = c as u8; // Writing character to VGA
    }
}