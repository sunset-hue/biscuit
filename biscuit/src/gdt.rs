
struct TableDescriptor {
    size: u16,
    location: u64
}

fn write_all_non_tss_segments(gdtr: TableDescriptor,tss_location: u64) {
    let mut address = gdtr.location as *mut u64;
    unsafe {    
        address.offset(0 as isize) = 0;
        address.offset(8 as isize) = 0x1000A3; //this is done to simplify the process of making segments
        address.offset(16 as isize) = 0x10009D;
        address.offset(24 as isize) = 0x100103;
        address.offset(32 as isize) = 0x1000FD;
    }
}

fn write_tss(gdtr: TableDescriptor,tss_location: u64) {
    let mut address = gdtr.location as *mut u64;
    unsafe { address.offset(40 as isize) = 0x0 }
    // writing a tss segment requires more bitfield work
}



