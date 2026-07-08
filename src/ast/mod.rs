#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // Declaración de función: fn nombre() { ... }
    FunctionDeclaration {
        name: String,
        body: Vec<Node>,
    },
    
    // Declaración de variable: let x = "valor"
    VariableDeclaration {
        name: String,
        value: String,
    },
    
    // Llamada a función o método: OS.display.print("...")
    FunctionCall {
        target: String,
        method: String,
        args: Vec<String>,
    },

    // Representación de un bloque de código
    Block {
        statements: Vec<Node>,
    },
}

