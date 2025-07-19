use limine::{framebuffer, BaseRevision};



fn print() {
    let revision = BaseRevision::new();
    let framebuffer = limine::request::FramebufferRequest::new();
    let mut response = framebuffer.get_response().unwrap().framebuffers()
                                        .last()
                                        .unwrap()
                                        .addr();
    
    // selecting last since I'm only gonna support one display rn
}