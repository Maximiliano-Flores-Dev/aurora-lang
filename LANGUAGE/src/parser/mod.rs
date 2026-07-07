pub mod ast;

use crate::parser::ast::{Stmt, Expr, Literal};

/// Analiza el cuerpo de la función "on_boot" y construye dinámicamente el AST
pub fn parse_source_code(raw_content: &str) -> Vec<Stmt> {
    let mut ast_tree = Vec::new();
    let mut inside_boot_fn = false;

    for line in raw_content.lines() {
        let trimmed = line.trim();

        // Detectar el inicio de la función de arranque
        if trimmed.starts_with("\"on_boot\": fn():") {
            inside_boot_fn = true;
            continue;
        }

        // Si estamos dentro de on_boot, procesamos sus expresiones
        if inside_boot_fn {
            // Si la línea cambia bruscamente de contexto o está vacía, la saltamos con cuidado
            if trimmed.is_empty() {
                continue;
            }
            
            // Si empieza con otra directiva principal, salimos del bloque de arranque
            if trimmed.starts_with("kernel_type:") {
                inside_boot_fn = false;
                continue;
            }

            // Detectar e interpretar llamadas al sistema: OS.modulo.funcion(...)
            if trimmed.starts_with("OS.") {
                if let Some(stmt) = parse_syscall_line(trimmed) {
                    ast_tree.push(stmt);
                }
            }
        }
    }

    ast_tree
}

/// Convierte una línea de texto plano de Aurora en un Nodo de Expresión Formal del AST
fn parse_syscall_line(line: &str) -> Option<Stmt> {
    // Ejemplo: OS.display.clear(color="black") o OS.display.init_screen()
    // Quitamos los espacios residuales
    let clean = line.trim();
    
    // Encontrar dónde abren y cierran los paréntesis de los argumentos
    let open_paren = clean.find('(')?;
    let close_paren = clean.find(')')?;
    
    let path_part = &clean[..open_paren]; // "OS.display.clear"
    let args_part = &clean[open_paren + 1..close_paren]; // "color=\"black\""

    // Separar el path por sus puntos
    let parts: Vec<&str> = path_part.split('.').collect();
    if parts.len() < 3 { return None; }

    let module = parts[0].to_string();    // OS
    let submodule = parts[1].to_string(); // display
    let action = parts[2].to_string();    // clear/init_screen

    // Parsear argumentos rudimentarios (por ahora detecta strings literales si existen)
    let mut args = Vec::new();
    if !args_part.is_empty() {
        if args_part.contains('=') {
            if let Some((arg_name, arg_val)) = args_part.split_once('=') {
                let val_clean = arg_val.replace('"', "").replace('\'', "");
                args.push((
                    arg_name.trim().to_string(),
                    Expr::Literal(Literal::String(val_clean.trim().to_string()))
                ));
            }
        } else {
            // Argumento posicional simple
            let val_clean = args_part.replace('"', "").replace('\'', "");
            args.push((
                "raw".to_string(),
                Expr::Literal(Literal::String(val_clean.trim().to_string()))
            ));
        }
    }

    Some(Stmt::Expression(Expr::SysCall {
        module,
        submodule,
        action,
        args,
    }))
}
