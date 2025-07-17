struct GateDescriptor {
    offset_1: u16,
    segment_selector: u16,
    ist: u8,
    gate_type: u8,
    dpl: u8,
    offset_2: u16,
    offset_3: u32
}
// this needs a struct because we need to serve IRQ's associated with the interrupts and having dedicated fields would be nice

fn write_idt_entries(address: u64) {
    let address = address as *mut u64;
    let gate_descriptor = 0x0;
    
    unsafe {
        for i in 0..255 {
                if i != 0x0 {
                    
            }
        } 
    }
}