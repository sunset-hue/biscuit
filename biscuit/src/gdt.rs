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
    privilege_level: u8, // most this can go up to is 3, so going to add upper bound
    descriptor_type: bool,
    executable: bool,
    dc: bool,
    rw: bool,
    // accessed bit is auto set to 1
    // present bit is auto set to 1
}

// long mode so no base/limits required, so hardcoded bases and limits are going to be used

fn encode_segment(flags: Flags,access: Access) {
    
}
