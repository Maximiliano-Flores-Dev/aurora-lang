pub mod storage;
pub mod display;
pub mod kernel;

use crate::config::KernelConfig;
use crate::parser::tokenizer::AuroraCommand;

pub fn orchestrate_codegen(config: &KernelConfig, commands: &[AuroraCommand]) -> String {
    let mut rust_code = String::new();
    
    rust_code.push_str("// Código modular de Aurora OS Compiler\n");
    rust_code.push_str("#![no_std]\n");
    rust_code.push_str("#![no_main]\n\n");
    rust_code.push_str("use core::panic::PanicInfo;\n\n");
    rust_code.push_str(&format!("// Config Target: {}, Versión: {}\n\n", config.target, config.version));

    // Runtime básico de interrupciones y memoria
    rust_code.push_str("#[no_mangle]\n");
    rust_code.push_str("pub unsafe extern \"C\" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {\n");
    rust_code.push_str("    for i in 0..n {\n");
    rust_code.push_str("        let a = *s1.add(i); let b = *s2.add(i);\n");
    rust_code.push_str("        if a != b { return (a as i32) - (b as i32); }\n");
    rust_code.push_str("    }\n");
    rust_code.push_str("    0\n");
    rust_code.push_str("}\n\n");
    rust_code.push_str("#[no_mangle]\npub extern \"C\" fn rust_eh_personality() {}\n\n");

    rust_code.push_str("#[allow(non_snake_case)]\n");
    rust_code.push_str("pub mod writer_core {\n");
    rust_code.push_str("    pub mod os {\n");
    rust_code.push_str("        pub mod OS {\n");

    // Inyectamos dinámicamente desde los archivos independientes del Kernel
    rust_code.push_str(&display::generate());
    rust_code.push_str(&storage::generate());
    rust_code.push_str(&kernel::generate());

    // HARDWARE, AGENTS stubs provisionales
    rust_code.push_str("            pub mod hardware {\n");
    rust_code.push_str("                pub unsafe fn bind_physical_button(_pin: i32, _duration_ms: i32, _trigger_event: unsafe fn()) {}\n");
    rust_code.push_str("            }\n");
    rust_code.push_str("            pub mod agents {\n");
    rust_code.push_str("                pub mod claw { pub unsafe fn awaken() {} }\n");
    rust_code.push_str("                pub mod rose { pub unsafe fn awaken() {} }\n");
    rust_code.push_str("            }\n");
    rust_code.push_str("            pub mod ui {\n");
    rust_code.push_str("                pub unsafe fn launch_desktop_environment() {}\n");
    rust_code.push_str("            }\n");

    rust_code.push_str("        }\n");
    rust_code.push_str("    }\n");
    rust_code.push_str("}\n\n");
    rust_code.push_str("use writer_core::os::OS;\n\n");

    // Bloque _start entry point basado en el AST del Tokenizer
    rust_code.push_str("#[no_mangle]\npub extern \"C\" fn _start() -> ! {\n    unsafe {\n");

    for cmd in commands {
        let mut rust_args = String::new();
        for (i, arg) in cmd.args.iter().enumerate() {
            if i > 0 { rust_args.push_str(", "); }
            
            // Si el argumento es una referencia a una función u objeto que empieza con "OS.",
            // transpilamos sus puntos (.) a dobles puntos (::) para que Rust lo entienda.
            if arg.starts_with("OS.") {
                rust_args.push_str(&arg.replace(".", "::"));
            } else {
                rust_args.push_str(arg);
            }
        }
        
        rust_code.push_str(&format!(
            "        OS::{}::{}({});\n",
            cmd.module, cmd.function, rust_args
        ));
    }

    rust_code.push_str("    }\n    loop {}\n}\n\n");
    rust_code.push_str("#[panic_handler]\nfn panic(_info: &PanicInfo) -> ! { loop {} }\n");

    rust_code
}
