pub fn generate() -> String {
    let mut code = String::new();
    code.push_str("            pub mod storage {\n");
    code.push_str("                use core::ptr::write_volatile;\n");
    code.push_str("                static mut HEAP_START: usize = 0x4010_0000;\n");
    code.push_str("                static mut HEAP_SIZE: usize = 0x00A0_0000;\n\n");
    code.push_str("                pub unsafe fn mount_secure_partition(_m: &str, _e: &str) {}\n\n");
    code.push_str("                pub unsafe fn allocate_memory_block(bytes: usize) -> *mut u8 {\n");
    code.push_str("                    let current_alloc = HEAP_START;\n");
    code.push_str("                    if current_alloc + bytes > HEAP_START + HEAP_SIZE {\n");
    code.push_str("                        return core::ptr::null_mut();\n");
    code.push_str("                    }\n");
    code.push_str("                    HEAP_START += bytes;\n");
    code.push_str("                    let ptr = current_alloc as *mut u8;\n");
    code.push_str("                    for i in 0..bytes { write_volatile(ptr.add(i), 0xAA); }\n");
    code.push_str("                    ptr\n");
    code.push_str("                }\n");
    code.push_str("            }\n");
    code
}
