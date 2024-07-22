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
    pub fn tokenize_true(input: &mut Peekable<Chars<'_>>) -> Token {
        input.next();
        assert_eq!(Some('r'), input.next());
        assert_eq!(Some('u'), input.next());
        assert_eq!(Some('e'), input.next());

        return Token::new(TokenKind::True, "true".to_string());
    }
    pub fn tokenize_false(input: &mut Peekable<Chars<'_>>) -> Token {
        input.next();
        assert_eq!(Some('a'), input.next());
        assert_eq!(Some('l'), input.next());
        assert_eq!(Some('s'), input.next());
        assert_eq!(Some('e'), input.next());

        return Token::new(TokenKind::False, "false".to_string());
    }
    pub fn tokenize_null(input: &mut Peekable<Chars<'_>>) -> Token {
        input.next();
        assert_eq!(Some('u'), input.next());
        assert_eq!(Some('l'), input.next());
        assert_eq!(Some('l'), input.next());

        return Token::new(TokenKind::Null, "null".to_string());
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
    pub fn tokenize_number(input: &mut Peekable<Chars<'_>>) -> Result<Token, String> {
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
                        eprintln!(
                            "Error: {:#?}",
                            "Using '.' two times in a single number is not allowed!"
                        );
                        std::process::exit(1); // exit the program on any occurance of this error
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
            match String::from_iter(value).parse::<f64>() {
                Ok(safe_value) => {
                    return Ok(Token::new(TokenKind::Number, safe_value.to_string()));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        } else {
            match String::from_iter(value).parse::<i64>() {
                Ok(safe_value) => {
                    return Ok(Token::new(TokenKind::Number, safe_value.to_string()));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
    }
}
