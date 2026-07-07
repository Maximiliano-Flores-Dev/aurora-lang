use std::process::Command;
use std::fs;

pub fn compile_elf_binary() {
    println!("[Pipeline] Generando binario base...");

    // Escribimos un wrapper Cargo temporal para mantener compatibilidad si se escala
    let cargo_toml_kernel = r#"
[package]
name = "kernel_output"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

    fs::write("Cargo.toml.tmp", cargo_toml_kernel).unwrap();

    // Invocar rustc rompiendo las cadenas del enlazador del sistema operativo anfitrión
    let status = Command::new("rustc")
        .arg("--crate-type=bin")
        .arg("--edition=2021")
        .arg("-C")
        .arg("opt-level=2")
        .arg("-C")
        .arg("panic=abort")
        // 💥 BANDERAS MAESTRAS DE ENLAZADO BARE-METAL:
        // Evita que el enlazador de Termux inyecte los archivos de inicio de Android/Linux (C Runtime)
        .arg("-C")
        .arg("link-arg=-nostartfiles")
        .arg("-C")
        .arg("link-arg=-nodefaultlibs")
        .arg("kernel_output.rs")
        .arg("-o")
        .arg("../OS/aurora_kernel.elf")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("\n=======================================================");
            println!("[ÉXITO TOTAL] ¡Binario ELF del OS guardado en /OS/!");
            println!("=======================================================");
        }
        _ => {
            println!("[Error Orquestador] Falló la compilación del binario base en Rust.");
        }
    }
    
    // Limpieza de archivos temporales
    let _ = fs::remove_file("Cargo.toml.tmp");
}
