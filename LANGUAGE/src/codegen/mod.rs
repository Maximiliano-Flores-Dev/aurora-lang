use crate::config::KernelConfig;
use crate::parser::ast::{Stmt, Expr, Literal};

pub fn orchestrate_codegen(config: &KernelConfig, ast_tree: &[Stmt]) -> String {
    let mut code = String::new();

    // 1. Cabecera Bare-Metal Soberana
    code.push_str("#![no_std]\n#![no_main]\n\n");
    code.push_str("use core::panic::PanicInfo;\n\n");
    
    // 2. Metadatos del Kernel inyectados
    code.push_str(&format!("// 🌌 KERNEL CONFIG: {}\n", config.display_name));
    code.push_str(&format!("// TARGET ARCH: {}\n\n", config.target));

    // 3. Mock Estructural en Rust Puro (Tolerante a cualquier firma)
    code.push_str(r#"
#[allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]
pub mod OS {
    pub mod display {
        pub fn init_screen() {}
        pub fn clear(color: &str) {}
        pub fn show_log(msg: &str) {}
        pub fn draw_boot_pattern() {}
    }

    pub mod storage {
        pub fn mount_secure_partition(args: &str) {}
    }

    pub mod hardware {
        pub fn bind_physical_button(args: &str) {}
    }

    pub mod kernel {
        pub fn start_scheduler() {}
    }

    pub mod ui {
        pub fn launch_desktop_environment() {}
    }

    pub mod agents {
        pub fn claw() {}
    }
}
"#);

    // 4. Punto de Entrada Principal (Corregido a un solo `#`)
    code.push_str("#[no_mangle]\npub extern \"C\" fn kernel_main() -> ! {\n");
    code.push_str("    // Inicializaciones automáticas generadas por el árbol sintáctico\n");
    
    // 5. Transpilar el AST real línea por línea
    for stmt in ast_tree {
        code.push_str(&format!("    {};\n", transpile_statement(stmt)));
    }

    code.push_str("\n    loop { core::hint::spin_loop(); }\n}\n\n");

    // 6. Panic Handler obligatorio
    code.push_str("#[panic_handler]\nfn panic(_info: &PanicInfo) -> ! {\n    loop {}\n}\n");

    code
}

fn transpile_statement(stmt: &Stmt) -> String {
    match stmt {
        Stmt::VarDeclaration { name, value_type: _, value } => {
            format!("let {} = {}", name, transpile_expression(value))
        }
        Stmt::Expression(expr) => transpile_expression(expr),
        Stmt::IfStatement { condition, then_branch, else_branch } => {
            let mut if_code = format!("if {} {{\n", transpile_expression(condition));
            for s in then_branch {
                if_code.push_str(&format!("        {};\n", transpile_statement(s)));
            }
            if let Some(else_s) = else_branch {
                if_code.push_str("    } else {\n");
                for s in else_s {
                    if_code.push_str(&format!("        {};\n", transpile_statement(s)));
                }
            }
            if_code.push_str("    }");
            if_code
        }
    }
}

fn transpile_expression(expr: &Expr) -> String {
    match expr {
        Expr::Literal(Literal::String(s)) => format!("\"{}\"", s),
        Expr::Literal(Literal::Int(i)) => i.to_string(),
        Expr::Literal(Literal::Bool(b)) => b.to_string(),
        Expr::Variable(v) => v.clone(),
        Expr::SysCall { module, submodule, action, args } => {
            let mut arg_strs = Vec::new();
            for (_, val) in args {
                arg_strs.push(transpile_expression(val));
            }
            if arg_strs.is_empty() {
                format!("{}::{}::{}()", module, submodule, action)
            } else {
                format!("{}::{}::{}({})", module, submodule, action, arg_strs.join(", "))
            }
        }
    }
}
