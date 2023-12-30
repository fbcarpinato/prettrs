#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Const,
    Identifier(&'a [u8]),
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

        println!("input: {:?}", input);

        let mut new_start = first_non_whitespace + 1;

        let token = match &input[0] {
            b'=' => {
                if input.starts_with(b"==") {
                    new_start += 1;
                    return Some(Token::DoubleEquals);
                } else {
                    return Some(Token::Equals);
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
            &c if c.is_ascii_digit() => {
                let end = input
                    .iter()
                    .position(|b| !b.is_ascii_digit())
                    .unwrap_or_else(|| input.len());

                let number_str = std::str::from_utf8(&self.input[..end])
                    .expect("Failed to convert bytes to string");

                let number = number_str.parse().expect("Expected a valid number");

                new_start += end - 1;
                return Some(Token::Number(number));
            }
            &c if c.is_ascii_alphabetic() => {
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
                println!("new_start: {}", new_start);
                return Some(token);
            }
            _ => None,
        };

        self.input = &self.input[new_start..];

        return token;
    }
}
