pub mod node;
pub mod token;
pub mod utils;

#[derive(Debug)]
pub struct JsonParser {}

impl JsonParser {
    pub fn new() -> JsonParser {
        return JsonParser {};
    }
    pub fn tokenize(&self, input: String) -> Result<Vec<token::Token>, String> {
        let mut tokens: Vec<token::Token> = vec![];

        let mut input = input.chars().peekable();

        while let Some(character) = input.peek() {
            match character {
                '{' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceOpen,
                        "{".to_string(),
                    ));
                    input.next();
                }
                '}' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceClose,
                        "}".to_string(),
                    ));
                    input.next();
                }
                '[' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketOpen,
                        "[".to_string(),
                    ));
                    input.next();
                }
                ']' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketClose,
                        "]".to_string(),
                    ));
                    input.next();
                }
                ',' => {
                    tokens.push(token::Token::new(token::TokenKind::Comma, ",".to_string()));
                    input.next();
                }
                ':' => {
                    tokens.push(token::Token::new(token::TokenKind::Colon, ":".to_string()));
                    input.next();
                }
                '"' => {
                    input.next();
                    tokens.push(token::Token::tokenize_string(&mut input));
                }
                '-' | '0'..='9' => match token::Token::tokenize_number(&mut input) {
                    Ok(safe_value) => {
                        tokens.push(safe_value);
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                't' => {
                    input.next();
                    assert_eq!(Some('r'), input.next());
                    assert_eq!(Some('u'), input.next());
                    assert_eq!(Some('e'), input.next());

                    tokens.push(token::Token::new(
                        token::TokenKind::True,
                        "true".to_string(),
                    ));
                }
                'f' => {
                    input.next();
                    assert_eq!(Some('a'), input.next());
                    assert_eq!(Some('l'), input.next());
                    assert_eq!(Some('s'), input.next());
                    assert_eq!(Some('e'), input.next());

                    tokens.push(token::Token::new(
                        token::TokenKind::False,
                        "false".to_string(),
                    ));
                }
                'n' => {
                    input.next();
                    assert_eq!(Some('u'), input.next());
                    assert_eq!(Some('l'), input.next());
                    assert_eq!(Some('l'), input.next());

                    tokens.push(token::Token::new(
                        token::TokenKind::Null,
                        "null".to_string(),
                    ));
                }
                '\0' => {
                    break;
                }
                other => {
                    if !other.is_whitespace() {
                        panic!("Unexpected token: {:#?}", other);
                    } else {
                        input.next();
                    }
                }
            }
        }
        return Ok(tokens);
    }

    pub fn parse(&self, input: String) -> Result<node::Node, String> {
        let tokens = match self.tokenize(input) {
            Ok(safe_value) => safe_value,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let mut value = node::Node::Null;

        let mut tokens = tokens.iter();
        while let Some(token) = tokens.next() {
            match token.kind {
                token::TokenKind::CurlyBraceOpen => {
                    value = node::Node::Object(match node::parse_object(&mut tokens) {
                        Ok(safe_value) => safe_value,
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    });
                }
                token::TokenKind::BracketOpen => {
                    value = node::Node::Array(match node::parse_array(&mut tokens) {
                        Ok(safe_value) => safe_value,
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    });
                }
                token::TokenKind::String => {
                    value = node::Node::String(token.value.clone());
                }
                token::TokenKind::Number => match token.value.parse::<f64>() {
                    Ok(safe_value) => {
                        value = node::Node::Number(safe_value);
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                token::TokenKind::True => {
                    value = node::Node::True;
                }
                token::TokenKind::False => {
                    value = node::Node::False;
                }
                token::TokenKind::Null => {
                    value = node::Node::Null;
                }
                // no need to handle them here
                token::TokenKind::CurlyBraceClose
                | token::TokenKind::BracketClose
                | token::TokenKind::Comma
                | token::TokenKind::Colon => {}
            }
        }
        return Ok(value);
    }
}
