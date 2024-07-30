pub mod node;
pub mod token;
pub mod utils;

#[derive(Debug)]
pub struct TomlParser {}

impl TomlParser {
    pub fn new() -> TomlParser {
        return TomlParser {};
    }
    pub fn tokenize(&self, input: String) -> Result<Vec<token::Token>, String> {
        let mut tokens: Vec<token::Token> = Vec::new();
        let mut input = input.chars().peekable();

        while let Some(character) = input.peek() {
            match character {
                '#' => {
                    while Some('\n') != input.peek().copied() {
                        input.next();
                    }
                }
                '{' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBracketOpen,
                        "{".to_string(),
                    ));
                    input.next();
                }
                '}' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::CurlyBracketOpen,
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
                '.' => {
                    tokens.push(token::Token::new(token::TokenKind::Dot, ".".to_string()));
                    input.next();
                }
                '=' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::EqualTo,
                        "=".to_string(),
                    ));
                    input.next();
                }
                '"' => {
                    input.next();
                    tokens.push(token::Token::tokenize_quote_string(&mut input));
                }
                '\n' => {
                    tokens.push(token::Token::new(
                        token::TokenKind::NewLine,
                        "\n".to_string(),
                    ));
                    input.next();
                }
                '\0' => {
                    break;
                }
                other => {
                    if other.is_whitespace() {
                        input.next();
                    } else if other.is_alphanumeric() || ['_', '-'].contains(other) {
                        match token::Token::tokenize_nonquote_string(&mut input) {
                            Ok(safe_value) => {
                                tokens.push(safe_value);
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    } else {
                        return Err(format!("Unexpected character: {:#?}", other));
                    }
                }
            }
        }
        return Ok(tokens);
    }
    pub fn parse(&self, input: Vec<token::Token>) -> Result<node::Node, String> {
        match utils::refine_tokens(input) {
            Ok(safe_value) => {
                let mut tokens = safe_value.iter().peekable();
                return Ok(node::Node::Table(node::parse_table(&mut tokens)));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
