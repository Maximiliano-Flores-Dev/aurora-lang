pub fn generate() -> String {
    let mut code = String::new();
    code.push_str("            pub mod kernel {\n");
    code.push_str("                use core::arch::asm;\n\n");
    
    code.push_str("                #[derive(Debug, Clone, Copy)]\n");
    code.push_str("                pub struct TaskContext {\n");
    code.push_str("                    pub r1: usize, pub sp: usize,\n");
    code.push_str("                }\n\n");

    code.push_str("                static mut CURRENT_CONTEXT: TaskContext = TaskContext { r1: 0, sp: 0 };\n\n");

    code.push_str("                #[no_mangle]\n");
    code.push_str("                pub unsafe extern \"C\" fn low_level_interrupt_vector() {\n");
    code.push_str("                    let mut out_r1: usize;\n");
    code.push_str("                    let mut out_sp: usize;\n");
    code.push_str("                    asm!(\n");
    code.push_str("                        \"mov {0}, x1\",\n"); // Usamos x1 (64-bits) en lugar de r1
    code.push_str("                        \"mov {1}, sp\",\n");
    code.push_str("                        out(reg) out_r1,\n");
    code.push_str("                        out(reg) out_sp,\n");
    code.push_str("                        options(nostack, nomem)\n");
    code.push_str("                    );\n");
    code.push_str("                    CURRENT_CONTEXT.r1 = out_r1;\n");
    code.push_str("                    CURRENT_CONTEXT.sp = out_sp;\n");
    code.push_str("                }\n\n");

    code.push_str("                pub unsafe fn start_scheduler() {\n");
    code.push_str("                    low_level_interrupt_vector();\n");
    code.push_str("                }\n");
    code.push_str("            }\n");
    code
}
