use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug)]
pub enum TokenKind {
    CurlyBraceOpen,
    CurlyBraceClose,
    BracketOpen,
    BracketClose,
    String,
    Number,
    Comma,
    Colon,
    True,
    False,
    Null,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        return Token { kind, value };
    }
    pub fn tokenize_string(input: &mut Peekable<Chars<'_>>) -> Token {
        let mut value = Vec::<char>::new();
        while let Some(character) = input.next() {
            if character == '"' {
                break;
            }
            value.push(character);
        }
        return Token::new(TokenKind::String, String::from_iter(value));
    }
    pub fn tokenize_number(input: &mut Peekable<Chars<'_>>) -> Token {
        let mut value = Vec::<char>::new();
        while input.peek().is_some_and(|peek_char| peek_char.is_numeric()) {
            if let Some(curr_char) = input.peek() {
                value.push(*curr_char);
            }
            let _ = input.next();
        }

        return Token::new(TokenKind::Number, String::from_iter(value));
    }
}
