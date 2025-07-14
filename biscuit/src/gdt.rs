// defining gdt structures and a function to make segments
struct TableDescriptor {
    size: u16,
    location: u64
}

struct Flags {
    granularity: bool,
    db: bool,
    long_mode: bool
}

struct Access {
    // required bits to set are gonna be automatically set in function
    privilege_level: u8,
    descriptor_type: bool,
    executable: bool,
    dc: bool,
    rw: bool,
    // accessed bit is auto set to 1
    // present bit is auto set to 1
}

// long mode so no base/limits required, so hardcoded bases and limits are going to be used

fn write_segment(address: u64,flags: Flags,access: Access) {
   if offset == 0 {
    return;
   }
   let addr_ptr = address as *mut u64;
   // pointer for memory address of segment descriptor in GDT
   // need to figure out how the hell to actually set bits inside entry (i'm confused)

}
