

fn main() {
    cc::Build::new()
    .file("loadgdt.asm")
    .compile("gdt_assembler");    
}