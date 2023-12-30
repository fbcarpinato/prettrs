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
                let boxed_const = Box::new(VariableStatement {
                    variable_declaration_list: Vec::new(),
                });
                statements.push(boxed_const);
            }
            _ => (),
        }
    }

    let root_node = ASTRootNode { statements };
    let ast = AST { root: root_node };

    return ast;
}
