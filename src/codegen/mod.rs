use crate::ast::Node;

pub fn generate_rust(node: Node) -> String {
    match node {
        Node::FunctionDeclaration { name, body: _ } => {
            format!("fn {}() {{ /* code here */ }}", name)
        }
        Node::VariableDeclaration { name, value } => {
            format!("let {} = {};", name, value)
        }
        _ => "// Unknown node".to_string(),
    }
}
