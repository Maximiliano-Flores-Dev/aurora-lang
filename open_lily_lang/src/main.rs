mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;

fn main() {
    // El código .oly que queremos procesar
    let input = "fn start() { }";
    
    // 1. Inicializamos el Lexer con nuestro código
    let lexer = Lexer::new(input);
    
    // 2. Pasamos el Lexer al Parser
    let mut parser = Parser::new(lexer);
    
    // 3. Ejecutamos el parseo y vemos el resultado
    let ast_node = parser.parse_function();
    
    println!("AST Generado: {:?}", ast_node);
}
