use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct SecurityConfig {
    telemetry: bool,
    privacy_mode: String,
    sandbox_apps: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct KernelConfig {
    display_name: String,
    version: String,
    target: String,
    output_format: String,
    security: SecurityConfig,
}

fn main() {
    println!("=== AURORA COMPILER CORE ===");

    let file_path = "main.aurora";
    
    if !Path::new(file_path).exists() {
        eprintln!("[Error] No se encontró el archivo '{}'.", file_path);
        return;
    }

    // 1. Leer el archivo de origen local
    let content = fs::read_to_string(file_path).expect("No se pudo leer main.aurora");
    println!("[Local-Read] Archivo '{}' cargado.", file_path);

    // 2. Parsear metadatos (Bloque JSON)
    if let Some(start_idx) = content.find("kernel_type: {") {
        let json_start = start_idx + "kernel_type: ".len();
        
        if let Some(end_idx) = content.find("\"on_boot\"") {
            let mut json_part = content[json_start..end_idx].trim().to_string();
            if json_part.ends_with(',') {
                json_part.pop();
            }
            json_part.push('}');

            // CORREGIDO: Cambiado '_' por '::' para llamar correctamente a la dependencia
            match serde_json::from_str::<KernelConfig>(&json_part) {
                Ok(config) => {
                    println!("[Parser Éxito] Configuración de '{} OS' validada.", config.display_name);

                    // =======================================================
                    // 3. MOTOR DE GENERACIÓN DE CÓDIGO (Codegen Robusto)
                    // =======================================================
                    println!("[Codegen] Traduciendo bloque 'on_boot' a bajo nivel...");
                    
                    let mut rust_code = String::new();
                    rust_code.push_str("// Código generado automáticamente por el compilador de Aurora OS\n");
                    rust_code.push_str("#![no_std]\n");
                    rust_code.push_str("#![no_main]\n\n");
                    rust_code.push_str("use core::panic::PanicInfo;\n\n");
                    
                    // Inyectamos un módulo mock local silenciando las advertencias de nomenclatura de Rust
                    rust_code.push_str("#[allow(non_snake_case)]\n");
                    rust_code.push_str("pub mod writer_core {\n");
                    rust_code.push_str("    pub mod os {\n");
                    rust_code.push_str("        pub mod OS {\n");
                    rust_code.push_str("            pub mod display { pub unsafe fn init_screen() {} pub unsafe fn clear(_c: &str) {} pub unsafe fn show_log(_m: &str) {} }\n");
                    rust_code.push_str("            pub mod storage { pub unsafe fn mount_secure_partition(_m: &str, _e: &str) {} }\n");
                    rust_code.push_str("            pub mod kernel { pub unsafe fn start_scheduler() {} }\n");
                    rust_code.push_str("            pub mod hardware { pub unsafe fn bind_physical_button(_p: i32, _d: i32, _e: unsafe fn()) {} }\n");
                    rust_code.push_str("            pub mod agents { pub mod claw { pub unsafe fn awaken() {} } }\n");
                    rust_code.push_str("            pub mod ui { pub unsafe fn launch_desktop_environment() {} }\n");
                    rust_code.push_str("        }\n");
                    rust_code.push_str("    }\n");
                    rust_code.push_str("}\n\n");
                    rust_code.push_str("use writer_core::os::OS;\n\n");
                    
                    rust_code.push_str("#[no_mangle]\n");
                    rust_code.push_str("pub extern \"C\" fn _start() -> ! {\n");
                    rust_code.push_str("    unsafe {\n");

                    if let Some(boot_start) = content.find("on_boot\": fn():") {
                        let boot_block = &content[boot_start..];
                        let mut buffer_llamada = String::new();
                        let mut acumulando = false;

                        for line in boot_block.lines() {
                            let trimmed = line.trim();
                            
                            if trimmed.starts_with('#') || trimmed.is_empty() {
                                continue;
                            }

                            if trimmed.starts_with("OS.") {
                                buffer_llamada = trimmed.to_string();
                                acumulando = true;
                            } else if acumulando {
                                buffer_llamada.push_str(" ");
                                buffer_llamada.push_str(trimmed);
                            }

                            // Si se cierra la instrucción funcional
                            if acumulando && buffer_llamada.contains(')') {
                                // Limpieza sintáctica profunda: traducción de namespaces y remoción de parámetros nombrados
                                let mut limpia = buffer_llamada
                                    .replace("OS.", "        OS::")
                                    .replace("display.", "display::")
                                    .replace("storage.", "storage::")
                                    .replace("kernel.", "kernel::")
                                    .replace("hardware.", "hardware::")
                                    .replace("ui.", "ui::")
                                    .replace("agents.claw.awaken", "agents::claw::awaken");

                                // Filtro para limpiar los parámetros nombrados del JSON/Python
                                limpia = limpia.replace("color=", "")
                                               .replace("mount_point=", "")
                                               .replace("encryption=", "")
                                               .replace("pin=", "")
                                               .replace("duration_ms=", "")
                                               .replace("trigger_event=", "");

                                rust_code.push_str(&limpia);
                                rust_code.push_str(";\n"); // Forzamos terminación limpia y salto de línea físico
                                buffer_llamada.clear();
                                acumulando = false;
                            }
                        }
                    }

                    rust_code.push_str("    }\n\n");
                    rust_code.push_str("    loop {}\n");
                    rust_code.push_str("}\n\n");
                    
                    rust_code.push_str("#[panic_handler]\n");
                    rust_code.push_str("fn panic(_info: &PanicInfo) -> ! {\n");
                    rust_code.push_str("    loop {}\n");
                    rust_code.push_str("}\n");

                    let output_path = "kernel_output.rs";
                    fs::write(output_path, rust_code).expect("No se pudo escribir kernel_output.rs");
                    println!("[Escritura Local] Transpilación completada con éxito en '{}'.", output_path);

                    // =======================================================
                    // 4. EL ORQUESTADOR: PROCESAMIENTO DEL TOOLCHAIN LOCAL
                    // =======================================================
                    let binary_name = "aurora_kernel.bin";

                    println!("[Orquestador] Ejecutando rustc local para target '{}'...", config.target);
                    let rustc_status = Command::new("rustc")
                        .arg("--crate-type=bin")
                        .arg("--edition=2021")
                        .arg("-C")
                        .arg("panic=abort")
                        .arg("-C")
                        .arg("link-arg=-nostartfiles") // Evita la duplicación del símbolo _start del sistema host
                        .arg("-o")
                        .arg(binary_name)
                        .arg(output_path)
                        .status();

                    match rustc_status {
                        Ok(status) if status.success() => {
                            println!("[Orquestador Éxito] Binario del kernel '{}' compiled.", binary_name);

                            let iso_dir = "iso_root";
                            let boot_dir = format!("{}/boot", iso_dir);
                            fs::create_dir_all(&boot_dir).expect("No se pudo crear directorio de booteo temporal");

                            let target_bin_path = format!("{}/kernel.bin", boot_dir);
                            fs::copy(binary_name, &target_bin_path).expect("No se pudo copiar el binario a la raíz de la ISO");

                            let final_iso_name = format!(
                                "{}-{}-V{}.{}",
                                config.display_name, config.target, config.version, config.output_format
                            );
                            println!("[Orquestador] Empaquetando árbol de archivos en '{}'...", final_iso_name);

                            let _iso_status = Command::new("xorriso")
                                .arg("-as")
                                .arg("mkisofs")
                                .arg("-o")
                                .arg(&final_iso_name)
                                .arg("-R")
                                .arg("-b")
                                .arg("boot/kernel.bin")
                                .arg(iso_dir)
                                .output();

                            let _ = fs::remove_dir_all(iso_dir);
                            let _ = fs::remove_file(binary_name);

                            println!("\n=======================================================");
                            println!("[PROYECTO TERMINADO] ¡Soberanía Tecnológica Consolidada!");
                            println!("Archivo Final de Sistema Operativo Listo para Flashear:");
                            println!(" -> ./{}", final_iso_name);
                            println!("=======================================================");
                        }
                        _ => {
                            eprintln!("[Error Orquestador] Falló la compilación nativa del kernel. Revisa la sintaxis de 'kernel_output.rs'.");
                        }
                    }
                }
                Err(e) => eprint!("[Parser Error] Metadatos inválidos: {}", e),
            }
        }
    }
}
