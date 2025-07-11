#![no_std] 
#![no_main]
static WSPPPP: &[u8] = b"Hello from BiscuitOS";


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buff = 0xb0000 as *mut u8;

    for (i,&byte) in WSPPPP.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = 0xb; 
        }
    }
    loop {}
}
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
