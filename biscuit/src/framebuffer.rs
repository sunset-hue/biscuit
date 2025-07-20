use limine::{file::File, framebuffer, BaseRevision};








 pub unsafe fn print(text: &str) {
    let revision = BaseRevision::new();
    let framebuffer = limine::request::FramebufferRequest::new();
    let mut response = framebuffer.get_response().unwrap().framebuffers().last().unwrap().addr();
    let bytes = include_bytes!("font.bin");
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    for char in text.chars() {
        let index = alphabet.iter().position(|x| *x==char).unwrap();
        
        *response.offset(0 as isize) = bytes[index * 128];
        // testing rn idk if this'll work or if I need to implement a parser

    }
}