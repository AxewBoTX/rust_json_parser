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

        let input = input.chars().collect::<Vec<char>>();
        let mut main_cursor_pos = 0;

        while main_cursor_pos < input.len() {
            match input[main_cursor_pos] {
                '{' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceOpen,
                        "{".to_string(),
                    ));
                }
                '}' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBraceClose,
                        "}".to_string(),
                    ));
                }
                '[' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketOpen,
                        "[".to_string(),
                    ));
                }
                ']' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::BracketClose,
                        "]".to_string(),
                    ));
                }
                ',' => {
                    tokens.push(token::Token::new(token::TokenKind::Comma, ",".to_string()));
                }
                ':' => {
                    tokens.push(token::Token::new(token::TokenKind::Colon, ":".to_string()));
                }
                '"' => {
                    let mut string_cursor_pos: Box<usize> = Box::new(main_cursor_pos);
                    let mut value = String::new();
                    while input[*string_cursor_pos + 1] != '"' {
                        if !input[*string_cursor_pos + 1].is_whitespace() {
                            value.push_str(input[*string_cursor_pos + 1].to_string().as_str());
                        }
                        *string_cursor_pos += 1;
                    }
                    tokens.push(token::Token::new(token::TokenKind::String, value));
                    main_cursor_pos = *string_cursor_pos + 1;
                }
                // ------ there is some problem here ------
                _ => {
                    if input[main_cursor_pos].is_alphanumeric() {
                        let mut value = String::new();
                        let mut alpha_cursor_pos: Box<usize> = Box::new(main_cursor_pos);
                        while input[*alpha_cursor_pos].is_alphanumeric() {
                            value.push_str(input[*alpha_cursor_pos].to_string().as_str());
                            *alpha_cursor_pos += 1;
                        }

                        if utils::is_number(value.as_str()) {
                            tokens.push(token::Token::new(
                                token::TokenKind::Number,
                                value.as_str().to_string(),
                            ));
                        }
                        if utils::is_bool_true(value.as_str()) {
                            tokens.push(token::Token::new(
                                token::TokenKind::True,
                                value.as_str().to_string(),
                            ));
                        }
                        if utils::is_bool_false(value.as_str()) {
                            tokens.push(token::Token::new(
                                token::TokenKind::False,
                                value.as_str().to_string(),
                            ));
                        }
                        if utils::is_null(value.as_str()) {
                            tokens.push(token::Token::new(
                                token::TokenKind::Null,
                                value.as_str().to_string(),
                            ));
                        }

                        main_cursor_pos = *alpha_cursor_pos - 1;
                    }
                }
            }
            main_cursor_pos += 1;
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
