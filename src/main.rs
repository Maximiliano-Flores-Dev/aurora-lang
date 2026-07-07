mod config;
mod parser;
mod codegen;
mod build_pipeline;

use std::fs;
use std::path::Path;
use crate::config::KernelConfig;

fn main() {
    println!("=== AURORA COMPILER CORE (PIPELINE INTEGRADO) ===");

    let file_path = "main.aurora";
    if !Path::new(file_path).exists() {
        eprintln!("[Error] No se encontró el archivo '{}'.", file_path);
        return;
    }

    let content = fs::read_to_string(file_path).expect("No se pudo leer main.aurora");
    println!("[Local-Read] Archivo '{}' cargado.", file_path);

    if let Some(start_idx) = content.find("kernel_type: {") {
        let json_start = start_idx + "kernel_type: ".len();
        if let Some(end_idx) = content.find("\"on_boot\"") {
            let json_part = content[json_start..end_idx].trim().rfind('}')
                .map(|idx| &content[json_start..json_start+idx+1])
                .unwrap_or(&content[json_start..end_idx]);

            if let Ok(config) = serde_json::from_str::<KernelConfig>(json_part) {
                println!("[Parser Éxito] Configuración de '{} OS' validada.", config.display_name);
                
                // Mapeo real sintáctico vía Tokenizer
                let commands = parser::tokenizer::tokenize_boot_block(&content);
                println!("[Análisis Sintáctico] {} comandos extraídos del bloque on_boot.", commands.len());

                // Se envían los comandos estructurados al generador de código
                let generated_rust = codegen::orchestrate_codegen(&config, &commands);
                let output_path = "kernel_output.rs";
                fs::write(output_path, generated_rust).expect("Error al escribir kernel_output.rs");
                println!("[Escritura Local] Transpilación modular completada.");

                if build_pipeline::compile_kernel(output_path) {
                    println!("\n=======================================================");
                    println!("[ÉXITO TOTAL] ¡Kernel y Driver de Texto Gráfico Listos!");
                    println!("=======================================================");
                } else {
                    eprintln!("[Error Orquestador] Falló la compilación del binario base.");
                }
            }
        }
    }
}
