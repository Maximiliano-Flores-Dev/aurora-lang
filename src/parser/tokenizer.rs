#[derive(Debug, Clone)]
pub struct AuroraCommand {
    pub module: String,
    pub function: String,
    pub args: Vec<String>,
}

pub fn tokenize_boot_block(raw_content: &str) -> Vec<AuroraCommand> {
    let mut commands = Vec::new();

    // Encontrar la firma de la función on_boot
    if let Some(boot_start) = raw_content.find("\"on_boot\": fn():") {
        // Tomamos todo lo que está después de la declaración de la función
        let block_content = &raw_content[boot_start + "\"on_boot\": fn():".len()..];
        
        // Procesamos línea por línea el cuerpo indentado
        for raw_line in block_content.lines() {
            let line = raw_line.trim();
            
            // Si la línea está vacía o es un comentario, la saltamos
            if line.is_empty() || line.starts_with('#') { continue; }
            
            // Si salimos del bloque de indentación de la función (u otra sección empieza), detenemos el parseo básico
            if !raw_line.starts_with("    ") && !line.starts_with("OS.") {
                // Si la línea no tiene espacios de indentación y no empieza con OS, asumimos fin de bloque
                if !line.is_empty() && !line.starts_with("OS.") { break; }
            }

            if line.starts_with("OS.") {
                if let Some(open_paren) = line.find('(') {
                    if let Some(close_paren) = line.rfind(')') {
                        let path = &line[3..open_paren]; // Extrae "display.init_screen"
                        let path_parts: Vec<&str> = path.split('.').collect();
                        
                        if path_parts.len() >= 2 {
                            let module = path_parts[0].to_string();
                            let function = path_parts[1].to_string();
                            
                            let raw_args = &line[open_paren + 1..close_paren];
                            let mut args = Vec::new();
                            
                            for arg in raw_args.split(',') {
                                let clean_arg = arg.trim();
                                if !clean_arg.is_empty() {
                                    // Separar clave=valor si existe (ej: color="black")
                                    let final_val = if let Some(eq_idx) = clean_arg.find('=') {
                                        clean_arg[eq_idx + 1..].trim().to_string()
                                    } else {
                                        clean_arg.to_string()
                                    };
                                    
                                    // Mantener el valor tal cual viene para el compilador Rust
                                    args.push(final_val);
                                }
                            }

                            commands.push(AuroraCommand { module, function, args });
                        }
                    }
                }
            }
        }
    }
    commands
}
