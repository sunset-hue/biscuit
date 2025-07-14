#![no_std] 
#![no_main]
mod gdt;

static WSPPPP: &[u8] = b"Hello from BiscuitOS";


#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    loop {}
}
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
