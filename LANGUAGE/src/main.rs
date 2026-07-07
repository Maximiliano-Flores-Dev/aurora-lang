mod config;
mod parser;
mod codegen;
mod build_pipeline;

use std::fs;

fn main() {
    println!("=== AURORA COMPILER CORE (PIPELINE INTEGRADO) ===");

    // Apuntamos quirúrgicamente a la nueva ubicación en EXAMPLES
    let aurora_file = "../EXAMPLES/boot_test.aurora";

    match fs::read_to_string(aurora_file) {
        Ok(content) => {
            println!("[Local-Read] Archivo '{}' cargado.", aurora_file);

            match config::parse_kernel_config(&content) {
                Ok(config) => {
                    println!("[Parser Éxito] Configuración de '{}' validada.", config.display_name);

                    let commands = parser::tokenizer::tokenize_boot_block(&content);
                    println!("[Análisis Sintáctico] {} comandos extraídos del bloque on_boot.", commands.len());

                    let generated_rust = codegen::orchestrate_codegen(&config, &commands);

                    if let Err(e) = fs::write("kernel_output.rs", &generated_rust) {
                        println!("[Error Escritura] No se pudo guardar kernel_output.rs: {}", e);
                        return;
                    }
                    println!("[Escritura Local] Transpilación modular completada.");

                    println!("[Orquestador] Compilando binario base ELF...");
                    build_pipeline::compile_elf_binary();
                }
                Err(e) => println!("[Error Config] {}", e),
            }
        }
        Err(_) => {
            println!("[Error Crítico] No se pudo encontrar el archivo '{}'. Asegúrate de que exista.", aurora_file);
        }
    }
}
