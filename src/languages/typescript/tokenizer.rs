#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Const,
    Identifier(&'a [u8]),
    TripleEquals,
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
    Dot,
    DoubleQuote,
    Return,
    Function,
    EOF,
}

pub struct Tokenizer<'a> {
    input: &'a [u8],
}

impl Tokenizer<'_> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input.as_bytes(),
        }
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        let first_non_whitespace = self
            .input
            .iter()
            .position(|&b| !b.is_ascii_whitespace())
            .unwrap_or_else(|| self.input.len());

        let input = &self.input[first_non_whitespace..];

        if input.is_empty() {
            return Some(Token::EOF);
        }

        let mut new_start = 1;

        let token: Option<Token> = match &input[0] {
            b'=' => {
                if input.starts_with(b"===") {
                    new_start += 2;
                    Some(Token::TripleEquals)
                } else if input.starts_with(b"==") {
                    new_start += 1;
                    Some(Token::DoubleEquals)
                } else {
                    Some(Token::Equals)
                }
            }
            b'{' => Some(Token::LeftBrace),
            b'}' => Some(Token::RightBrace),
            b'(' => Some(Token::LeftParen),
            b')' => Some(Token::RightParen),
            b';' => Some(Token::Semicolon),
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Minus),
            b'/' => Some(Token::Slash),
            b'*' => Some(Token::Star),
            b',' => Some(Token::Comma),
            b'.' => Some(Token::Dot),
            b'"' => Some(Token::DoubleQuote),
            b'1'..=b'9' => {
                let end = input
                    .iter()
                    .position(|b| !b.is_ascii_digit())
                    .unwrap_or_else(|| input.len());

                let number_str =
                    std::str::from_utf8(&input[..end]).expect("Failed to convert bytes to string");

                let number = number_str.parse().expect("Expected a valid number");

                new_start += end - 1;
                Some(Token::Number(number))
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                let end = input
                    .iter()
                    .position(|b| !b.is_ascii_alphabetic())
                    .unwrap_or_else(|| input.len());

                let token = match &input[..end] {
                    b"const" => Token::Const,
                    b"if" => Token::If,
                    b"return" => Token::Return,
                    b"function" => Token::Function,
                    identifier => Token::Identifier(identifier),
                };

                new_start += end - 1;
                Some(token)
            }
            _ => Some(Token::EOF),
        };

        self.input = &input[new_start..];

        token
    }
}
