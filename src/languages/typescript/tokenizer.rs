#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Const,
    Identifier(&'a str),
    DoubleEquals,
    Equals,
    Number(i32),
    Semicolon,
    If,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Slash,
    Star,
    Comma,
    Return,
    Function,
}

type TokenizationError = String;
pub type TokenizationResult<'a> = Result<Vec<Token<'a>>, TokenizationError>;

fn next_token(input: &str) -> Option<(Token, &str)> {
    let input = input.trim_start();
    let bytes = input.as_bytes();

    match bytes.first()? {
        b'=' => {
            if input.starts_with("==") {
                Some((Token::DoubleEquals, &input[2..]))
            } else {
                Some((Token::Equals, &input[1..]))
            }
        }
        b'{' => Some((Token::LeftBrace, &input[1..])),
        b'}' => Some((Token::RightBrace, &input[1..])),
        b'(' => Some((Token::LeftParen, &input[1..])),
        b')' => Some((Token::RightParen, &input[1..])),
        b';' => Some((Token::Semicolon, &input[1..])),
        b'+' => Some((Token::Plus, &input[1..])),
        b'-' => Some((Token::Minus, &input[1..])),
        b'/' => Some((Token::Slash, &input[1..])),
        b'*' => Some((Token::Star, &input[1..])),
        b',' => Some((Token::Comma, &input[1..])),
        &c if c.is_ascii_digit() => {
            let end = bytes
                .iter()
                .position(|b| !b.is_ascii_digit())
                .unwrap_or_else(|| input.len());

            let number = input[..end].parse().expect("number was expected");

            Some((Token::Number(number), &input[end..]))
        }
        &c if c.is_ascii_alphabetic() => {
            let end = bytes
                .iter()
                .position(|b| !b.is_ascii_alphabetic())
                .unwrap_or_else(|| input.len());

            let token = match &input[..end] {
                "const" => Token::Const,
                "if" => Token::If,
                "return" => Token::Return,
                "function" => Token::Function,
                identifier => Token::Identifier(identifier),
            };

            Some((token, &input[end..]))
        }
        _ => None,
    }
}

pub fn tokenize(input: &str) -> TokenizationResult {
    let mut tokens = Vec::new();

    let mut input = input;

    while let Some((token, remaining)) = next_token(input) {
        tokens.push(token);
        input = remaining;
    }

    return Ok(tokens);
}

#[cfg(test)]
mod test {
    use super::tokenize;
    use super::Token;

    #[test]
    fn test_tokenize_const_declaration() {
        let tokens = tokenize("const x = 1;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Const,
                Token::Identifier("x"),
                Token::Equals,
                Token::Number(1),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_tokenize_const_declaration_without_spaces() {
        let tokens = tokenize("const x=1;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Const,
                Token::Identifier("x"),
                Token::Equals,
                Token::Number(1),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_tokenize_if_statement() {
        let tokens = tokenize("if (x == 1) { x = 2; }").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::LeftParen,
                Token::Identifier("x"),
                Token::DoubleEquals,
                Token::Number(1),
                Token::RightParen,
                Token::LeftBrace,
                Token::Identifier("x"),
                Token::Equals,
                Token::Number(2),
                Token::Semicolon,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn test_tokenize_if_statement_without_spaces() {
        let tokens = tokenize("if(x==1){x=2;}").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::LeftParen,
                Token::Identifier("x"),
                Token::DoubleEquals,
                Token::Number(1),
                Token::RightParen,
                Token::LeftBrace,
                Token::Identifier("x"),
                Token::Equals,
                Token::Number(2),
                Token::Semicolon,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn test_tokenize_if_statement_with_newlines() {
        let tokens = tokenize("if(x==1){\n  x=2;\n}").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::LeftParen,
                Token::Identifier("x"),
                Token::DoubleEquals,
                Token::Number(1),
                Token::RightParen,
                Token::LeftBrace,
                Token::Identifier("x"),
                Token::Equals,
                Token::Number(2),
                Token::Semicolon,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn test_tokenize_function_declaration() {
        let tokens = tokenize("function add(x, y) { return x + y; }").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Function,
                Token::Identifier("add"),
                Token::LeftParen,
                Token::Identifier("x"),
                Token::Comma,
                Token::Identifier("y"),
                Token::RightParen,
                Token::LeftBrace,
                Token::Return,
                Token::Identifier("x"),
                Token::Plus,
                Token::Identifier("y"),
                Token::Semicolon,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn test_tokenize_sum_operation() {
        let tokens = tokenize("1 + 4 = 5;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(4),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_tokenize_minus_operation() {
        let tokens = tokenize("1 - 4 = -3;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Minus,
                Token::Number(4),
                Token::Equals,
                Token::Minus,
                Token::Number(3),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_tokenize_division_operation() {
        let tokens = tokenize("4 / 1 = 4;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(4),
                Token::Slash,
                Token::Number(1),
                Token::Equals,
                Token::Number(4),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_tokenize_multiplication_operation() {
        let tokens = tokenize("4 * 1 = 4;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(4),
                Token::Star,
                Token::Number(1),
                Token::Equals,
                Token::Number(4),
                Token::Semicolon
            ]
        );
    }
}
