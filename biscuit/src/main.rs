#![no_std] 
#![no_main]
#![feature(global_asm)]
mod gdt;
mod tss;

extern "C" {
    fn setGdt(limit:u64,base:u64);
}




#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    unsafe {    
        gdt::write_all_non_tss_segments(gdt::TableDescriptor {size: 32-1,location:0x3});
        setGdt(0x20 - 1,0xC0010000);
        // the setGdt logic is from the GDT tutorial, i'm guessing the limit means the size, and location is where gdt is going to be located
    }
    loop {}
}
use core::{arch::global_asm, panic::PanicInfo};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
