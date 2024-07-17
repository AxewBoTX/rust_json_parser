pub mod node;
pub mod token;
pub mod utils;

#[derive(Debug)]
pub struct JsonParser {}

impl JsonParser {
    pub fn new() -> JsonParser {
        return JsonParser {};
    }
    pub fn tokenize(&self, input: String) -> Vec<token::Token> {
        let mut tokens: Vec<token::Token> = vec![];

        let mut input = input.chars().peekable();

        while let Some(character) = input.peek() {
            match character {
                '{' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceOpen,
                        "{".to_string(),
                    ));
                    let _ = input.next();
                }
                '}' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceClose,
                        "}".to_string(),
                    ));
                    let _ = input.next();
                }
                '[' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketOpen,
                        "[".to_string(),
                    ));
                    let _ = input.next();
                }
                ']' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketClose,
                        "]".to_string(),
                    ));
                    let _ = input.next();
                }
                ',' => {
                    tokens.push(token::Token::new(token::TokenKind::Comma, ",".to_string()));
                    let _ = input.next();
                }
                ':' => {
                    tokens.push(token::Token::new(token::TokenKind::Colon, ":".to_string()));
                    let _ = input.next();
                }
                '"' => {
                    let _ = input.next();
                    tokens.push(token::Token::tokenize_string(&mut input));
                }
                '-' | '0'..='9' => {
                    tokens.push(token::Token::tokenize_number(&mut input));
                }
                't' => {
                    let _ = input.next();
                    assert_eq!(Some('r'), input.next());
                    assert_eq!(Some('u'), input.next());
                    assert_eq!(Some('e'), input.next());

                    tokens.push(token::Token::new(
                        token::TokenKind::True,
                        "true".to_string(),
                    ));
                }
                'f' => {
                    let _ = input.next();
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
                    let _ = input.next();
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
        return tokens;
    }

    pub fn parse(&self, tokens: Vec<token::Token>) -> node::Node {
        let mut value = node::Node::Null;

        let mut tokens = tokens.iter();
        while let Some(token) = tokens.next() {
            match token.kind {
                token::TokenKind::CurlyBraceOpen => {
                    value = node::Node::Object(node::parse_object(&mut tokens));
                }
                token::TokenKind::BracketOpen => {
                    value = node::Node::Array(node::parse_array(&mut tokens));
                }
                token::TokenKind::String => {
                    value = node::Node::String(token.value.clone());
                }
                token::TokenKind::Number => {
                    value = node::Node::Number(token.value.parse::<f64>().unwrap());
                }
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
        return value;
    }
}
