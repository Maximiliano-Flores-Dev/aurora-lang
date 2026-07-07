use std::process::Command;
use std::fs;

pub fn compile_kernel(output_path: &str) -> bool {
    let binary_elf = "aurora_kernel.elf";
    let binary_raw = "kernel.bin";

    println!("[Orquestador] Compilando binario base ELF...");
    let rustc_status = Command::new("rustc")
        .arg("--crate-type=bin")
        .arg("--edition=2021")
        .arg("-C")
        .arg("panic=abort")
        .arg("-C")
        .arg("relocation-model=static")
        .arg("-C")
        .arg("link-arg=-nostartfiles")
        .arg("-o")
        .arg(binary_elf)
        .arg(output_path)
        .status();

    match rustc_status {
        Ok(status) if status.success() => {
            println!("[Orquestador Éxito] Ejecutable ELF creado.");
            let objcopy_status = Command::new("objcopy")
                .arg("-O")
                .arg("binary")
                .arg(binary_elf)
                .arg(binary_raw)
                .status();

            let _ = fs::remove_file(binary_elf);

            objcopy_status.is_ok() && objcopy_status.unwrap().success()
        }
        _ => false
    }
}
