use std::fmt::Debug;

use super::tokenizer::Tokenizer;

pub trait Statement: Debug {}

#[derive(Debug)]
struct IfStatement;
impl Statement for IfStatement {}

#[derive(Debug)]
struct VariableDeclaration {
    identifier: String,
    value: String,
}

#[derive(Debug)]
struct VariableStatement {
    variable_declaration_list: Vec<VariableDeclaration>,
}
impl Statement for VariableStatement {}

#[derive(Debug)]
pub struct ASTRootNode {
    statements: Vec<Box<dyn Statement>>,
}

#[derive(Debug)]
pub struct AST {
    root: ASTRootNode,
}

pub fn build_ast(input: &str) -> AST {
    let input = &mut input.to_string();
    let mut tokenizer = Tokenizer::new(input);

    let mut statements: Vec<Box<dyn Statement>> = Vec::new();

    while let Some(token) = tokenizer.consume_token() {
        match token {
            super::tokenizer::Token::If => {
                let boxed_if = Box::new(IfStatement);
                statements.push(boxed_if);
            }
            super::tokenizer::Token::Const => {
                let identifier_token = tokenizer.consume_token();

                let mut variable_declaration = VariableDeclaration {
                    identifier: String::new(),
                    value: String::new(),
                };

                match identifier_token {
                    Some(super::tokenizer::Token::Identifier(identifier)) => {
                        variable_declaration.identifier = String::from_utf8_lossy(identifier)
                            .to_string()
                            .to_lowercase();
                    }
                    _ => {
                        panic!("Expected identifier after const");
                    }
                }

                let equals_token = tokenizer.consume_token();

                match equals_token {
                    Some(super::tokenizer::Token::Equals) => (),
                    _ => {
                        panic!("Expected equals after identifier");
                    }
                } 
                
                let value_token = tokenizer.consume_token();

                match value_token {
                    Some(super::tokenizer::Token::Number(value)) => {
                        variable_declaration.value = value.to_string();
                    }
                    _ => {
                        panic!("Expected number after equals");
                    }
                }

                let semicolon_token = tokenizer.consume_token();

                match semicolon_token {
                    Some(super::tokenizer::Token::Semicolon) => (),
                    _ => {
                        panic!("Expected semicolon after number");
                    }
                }

                let statement = Box::new(VariableStatement {
                    variable_declaration_list: Vec::from([variable_declaration]) 
                });
                statements.push(statement);
            }
            super::tokenizer::Token::EOF => break,
            _ => (),
        }
    }

    let root_node = ASTRootNode { statements };
    let ast = AST { root: root_node };

    return ast;
}
