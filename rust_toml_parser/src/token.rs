use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum TokenKind {
    CurlyBracketOpen,
    CurlyBracketClose,
    BracketOpen,
    BracketClose,
    EqualTo,
    Dot,
    Comma,
    String,
    Number,
    True,
    False,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        return Token { kind, value };
    }
    pub fn tokenize_quote_string(input: &mut Peekable<Chars<'_>>) -> Token {
        let mut value = Vec::<char>::new();
        while let Some(character) = input.next() {
            if character == '"' {
                break;
            }
            value.push(character);
        }
        return Token::new(TokenKind::String, String::from_iter(value));
    }
    pub fn tokenize_nonquote_string(input: &mut Peekable<Chars<'_>>) -> Token {
        let mut value = Vec::<char>::new();
        if input.peek().is_some_and(|character| character.is_numeric()) {
            input.next();
        }
        while let Some(character) = input.peek() {
            if character.is_alphanumeric() {
                value.push(*character);
                input.next();
            } else {
                break;
            }
        }
        if [String::from("true"), String::from("false")].contains(&String::from_iter(value.clone()))
        {
            while let Some(character) = input.peek() {
                match character {
                    '=' => {
                        break;
                    }
                    '\n' => {
                        if String::from_iter(value.clone()) == String::from("true") {
                            return Token::new(TokenKind::True, String::from("true"));
                        }
                        if String::from_iter(value.clone()) == String::from("false") {
                            return Token::new(TokenKind::False, String::from("false"));
                        }
                    }
                    other => {
                        if other.is_whitespace() {
                            input.next();
                        }
                    }
                }
            }
        }
        return Token::new(TokenKind::String, String::from_iter(value.clone()));
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
                '}' | ',' | ']' | '=' => {
                    break;
                }
                '\n' => {
                    break;
                }
                other => {
                    if !other.is_ascii_whitespace() {
                        eprintln!("Unexpected character while parsing number: {character}");
                    }
                    input.next();
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
