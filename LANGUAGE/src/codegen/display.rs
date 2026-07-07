pub fn generate() -> String {
    let mut code = String::new();
    code.push_str("            pub mod display {\n");
    code.push_str("                use core::ptr::write_volatile;\n");
    code.push_str("                const FRAMEBUFFER_BASE: *mut u32 = 0x4000_0000 as *mut u32;\n");
    code.push_str("                const SCREEN_WIDTH: usize = 800;\n\n");
    
    // Matriz simplificada de fuentes bitmap 8x8 (ejemplo básico para caracteres esenciales: A, B, S, O, espacio, etc.)
    code.push_str("                const FONT_BITMAPS: [[u8; 8]; 6] = [\n");
    code.push_str("                    [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00], // 'A'\n");
    code.push_str("                    [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00], // 'B'\n");
    code.push_str("                    [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x66, 0x3C, 0x00], // 'S'\n");
    code.push_str("                    [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00], // 'O'\n");
    code.push_str("                    [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // 'T'\n");
    code.push_str("                    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]  // Espacio\n");
    code.push_str("                ];\n\n");

    code.push_str("                pub unsafe fn init_screen() {}\n\n");
    
    code.push_str("                pub unsafe fn clear(color: &str) {\n");
    code.push_str("                    let raw_color = if color == \"black\" { 0x00000000 } else { 0xFFFFFFFF };\n");
    code.push_str("                    for i in 0..480000 { write_volatile(FRAMEBUFFER_BASE.add(i), raw_color); }\n");
    code.push_str("                }\n\n");

    code.push_str("                pub unsafe fn draw_boot_pattern() {\n");
    code.push_str("                    for i in 0..20000 {\n");
    code.push_str("                        let color = if (i / 128) % 2 == 0 { 0x000000FF } else { 0x0000FF00 };\n");
    code.push_str("                        write_volatile(FRAMEBUFFER_BASE.add(i), color);\n");
    code.push_str("                    }\n");
    code.push_str("                }\n\n");

    // Función que lee texto dinámico de Aurora y dibuja los bitmaps mapeados por hardware
    code.push_str("                pub unsafe fn show_log(text: &str) {\n");
    code.push_str("                    let mut start_x = 20;\n");
    code.push_str("                    let start_y = 100;\n");
    code.push_str("                    for c in text.chars() {\n");
    code.push_str("                        let glyph = match c {\n");
    code.push_str("                            'A' => &FONT_BITMAPS[0],\n");
    code.push_str("                            'B' => &FONT_BITMAPS[1],\n");
    code.push_str("                            'S' => &FONT_BITMAPS[2],\n");
    code.push_str("                            'O' => &FONT_BITMAPS[3],\n");
    code.push_str("                            'T' => &FONT_BITMAPS[4],\n");
    code.push_str("                            _   => &FONT_BITMAPS[5],\n");
    code.push_str("                        };\n");
    code.push_str("                        for row in 0..8 {\n");
    code.push_str("                            let bits = glyph[row];\n");
    code.push_str("                            for col in 0..8 {\n");
    code.push_str("                                if (bits & (1 << (7 - col))) != 0 {\n");
    code.push_str("                                    let offset = (start_y + row) * SCREEN_WIDTH + (start_x + col);\n");
    code.push_str("                                    write_volatile(FRAMEBUFFER_BASE.add(offset), 0x00FFFF00);\n");
    code.push_str("                                }\n");
    code.push_str("                            }\n");
    code.push_str("                        }\n");
    code.push_str("                        start_x += 10;\n");
    code.push_str("                    }\n");
    code.push_str("                }\n");
    code.push_str("            }\n");
    code
}
