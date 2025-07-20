
pub struct TableDescriptor {
    pub size: u16,
    pub location: u64
}

pub fn write_all_non_tss_segments(gdtr: TableDescriptor) {
    let mut address = gdtr.location as *mut u64;
    unsafe {    
        *address.offset(0 as isize) = 0;
        *address.offset(8 as isize) = 0x1000A3; //this is done to simplify the process of making segments
        *address.offset(16 as isize) = 0x10009D;
        *address.offset(24 as isize) = 0x100103;
        *address.offset(32 as isize) = 0x1000FD;
    }
}



