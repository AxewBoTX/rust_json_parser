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
        let mut is_decimal = false;
        while let Some(character) = input.peek() {
            match character {
                '-' => {
                    value.push('-');
                    input.next();
                }
                '+' => {
                    input.next();
                }
                digit @ '0'..='9' => {
                    value.push(*digit);
                    input.next();
                }
                '.' => {
                    if is_decimal == false {
                        value.push('.');
                        is_decimal = true;
                    } else {
                        println!("Using '.' two times in a single number is not allowed!");
                    }
                    input.next();
                }
                '}' | ',' | ']' | ':' => {
                    break;
                }
                other => {
                    if !other.is_ascii_whitespace() {
                        panic!("Unexpected character while parsing number: {character}");
                    } else {
                        input.next();
                    }
                }
            }
        }

        if is_decimal {
            return Token::new(
                TokenKind::Number,
                String::from_iter(value).parse::<f64>().unwrap().to_string(),
            );
        } else {
            return Token::new(
                TokenKind::Number,
                String::from_iter(value).parse::<i64>().unwrap().to_string(),
            );
        }
    }
}
