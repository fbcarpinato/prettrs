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

pub fn parse_const_ast(tokenizer: &mut Tokenizer) -> Result<Box<dyn Statement>, String> {
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
            return Err("Expected identifier after const".to_string());
        }
    }

    let equals_token = tokenizer.consume_token();

    match equals_token {
        Some(super::tokenizer::Token::Equals) => (),
        _ => {
            return Err("Expected equals after identifier".to_string());
        }
    }

    let value_token = tokenizer.consume_token();

    match value_token {
        Some(super::tokenizer::Token::Number(value)) => {
            variable_declaration.value = value.to_string();
        }
        _ => {
            return Err("Expected number after equals".to_string());
        }
    }

    let semicolon_token = tokenizer.consume_token();

    match semicolon_token {
        Some(super::tokenizer::Token::Semicolon) => (),
        _ => {
            return Err("Expected semicolon after number".to_string());
        }
    }

    return Ok(Box::new(VariableStatement {
        variable_declaration_list: Vec::from([variable_declaration]),
    }));
}

pub fn parse_if_statement_ast(tokenizer: &mut Tokenizer) -> Result<Box<dyn Statement>, String> {
    let left_parenthesis_token = tokenizer.consume_token();

    match left_parenthesis_token {
        Some(super::tokenizer::Token::LeftParen) => (),
        _ => {
            return Err("Expected left parenthesis after if".to_string());
        }
    }

    while let Some(token) = tokenizer.consume_token() {
        println!("Token: {:?}", token);
        match token {
            super::tokenizer::Token::RightParen => break,
            super::tokenizer::Token::EOF => {
                return Err("Expected right parenthesis after if".to_string());
            }
            super::tokenizer::Token::Identifier(ident) => {
                println!("Identifier: {}", String::from_utf8_lossy(ident));
            }
            _ => (),
        }
    }

    #[allow(unused_variables)]
    let left_brace_token = tokenizer.consume_token();

    println!("Left brace token: {:?}", left_brace_token);

    while let Some(token) = tokenizer.consume_token() {
        println!("Token: {:?}", token);
        match token {
            super::tokenizer::Token::RightBrace => break,
            super::tokenizer::Token::EOF => {
                return Err("Expected right brace after if".to_string());
            }
            super::tokenizer::Token::Identifier(ident) => {
                println!("Identifier: {}", String::from_utf8_lossy(ident));
            }
            _ => (),
        }
    }

    return Ok(Box::new(IfStatement {}));
}

pub fn build_ast(input: String) -> AST {
    let mut tokenizer = Tokenizer::new(&input);

    let mut statements: Vec<Box<dyn Statement>> = Vec::new();

    loop {
        let token = tokenizer.consume_token();

        match token {
            Some(super::tokenizer::Token::If) => {
                let statement = parse_if_statement_ast(&mut tokenizer);

                match statement {
                    Ok(statement) => statements.push(statement),
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            Some(super::tokenizer::Token::Const) => {
                let statement = parse_const_ast(&mut tokenizer);

                match statement {
                    Ok(statement) => statements.push(statement),
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            Some(super::tokenizer::Token::EOF) => break,
            _ => (),
        }
    }

    let root_node = ASTRootNode { statements };
    let ast = AST { root: root_node };

    return ast;
}
