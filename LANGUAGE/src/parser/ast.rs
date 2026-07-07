#[derive(Debug, Clone)]
pub enum Type {
    String,
    Int,
    Bool,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Variable(String),
    Literal(Literal),
    SysCall {
        module: String,
        submodule: String,
        action: String,
        args: Vec<(String, Expr)>, // Argumentos nombrados como (nombre, valor)
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDeclaration {
        name: String,
        value_type: Type,
        value: Expr,
    },
    IfStatement {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Expression(Expr),
}
