use crate::lexer::{Token, Lexer};
use crate::ast::Node;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let first_token = lexer.next_token();
        Self { lexer, current_token: first_token }
    }

    fn eat(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Error de sintaxis: esperado {:?}", expected);
        }
    }

    pub fn parse_function(&mut self) -> Node {
        self.eat(Token::Fn);
        if let Token::Identifier(name) = self.current_token.clone() {
            self.current_token = self.lexer.next_token();
            self.eat(Token::OpenParen);
            self.eat(Token::CloseParen);
            self.eat(Token::OpenBrace);
            
            // Aquí parsearíamos el cuerpo de la función...
            self.eat(Token::CloseBrace);
            
            Node::FunctionDeclaration { name, body: vec![] }
        } else {
            panic!("Error: esperado nombre de función");
        }
    }
}
