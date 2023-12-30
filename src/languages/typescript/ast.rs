use std::fmt::Debug;

pub trait Statement: Debug {}

#[derive(Debug)]
struct IfStatement;
impl Statement for IfStatement {}

#[derive(Debug)]
struct VariableStatement;
impl Statement for VariableStatement {}

#[derive(Debug)]
pub struct ASTRootNode {
    statements: Vec<Box<dyn Statement>>,
}

#[derive(Debug)]
pub struct AST {
    root: ASTRootNode,
}

pub fn build_ast() -> AST {
    let if_statement = IfStatement;
    let variable_statement = VariableStatement;

    let boxed_if = Box::new(if_statement);
    let boxed_variable = Box::new(variable_statement);

    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    statements.push(boxed_if);
    statements.push(boxed_variable);

    let root_node = ASTRootNode { statements };
    let ast = AST { root: root_node };

    return ast;
}
