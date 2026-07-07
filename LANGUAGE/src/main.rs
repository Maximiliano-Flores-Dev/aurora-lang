mod config;
mod parser;
mod codegen;
mod build_pipeline;

use std::fs;

fn main() {
    println!("=== AURORA COMPILER CORE (PROCESAMIENTO AST COMPLETO) ===");

    let aurora_file = "../EXAMPLES/boot_test.aurora";

    match fs::read_to_string(aurora_file) {
        Ok(content) => {
            // 1. Parsear los Metadatos del archivo de configuración nativo
            match config::parse_kernel_config(&content) {
                Ok(config) => {
                    println!("[Config] Cargados metadatos del Kernel: {}", config.display_name);

                    // 2. EXTRAER EL AST REAL DESDE EL CÓDIGO FUENTE (¡Adiós simulación!)
                    println!("[Parser] Analizando el código fuente de Aurora dinámicamente...");
                    let real_ast = parser::parse_source_code(&content);
                    
                    println!("[AST Éxito] Se estructuraron dinámicamente {} nodos sintácticos desde el archivo.", real_ast.len());

                    if real_ast.is_empty() {
                        println!("[Alerta] No se encontraron comandos válidos dentro del bloque 'on_boot'.");
                    }

                    // 3. Orquestar la generación de código Rust bare-metal
                    let generated_rust = codegen::orchestrate_codegen(&config, &real_ast);

                    // 4. Guardar la salida transpilada
                    if let Err(e) = fs::write("kernel_output.rs", &generated_rust) {
                        println!("[Error] No se pudo escribir kernel_output.rs: {}", e);
                        return;
                    }
                    println!("[Codegen] Código intermedio unificado en 'kernel_output.rs'.");

                    // 5. Compilar el ejecutable final del OS
                    build_pipeline::compile_elf_binary();
                }
                Err(e) => println!("[Error Config] {}", e),
            }
        }
        Err(_) => println!("[Error Crítico] No se encontró el archivo origen '{}'.", aurora_file),
    }
}
