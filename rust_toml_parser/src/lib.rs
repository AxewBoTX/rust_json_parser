pub mod node;
pub mod token;

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
                '\0' => {
                    break;
                }
                other => {
                    if other.is_whitespace() {
                        input.next();
                    } else if other.is_alphanumeric() || ['_', '-'].contains(other) {
                        tokens.push(token::Token::tokenize_nonquote_string(&mut input))
                    } else {
                        panic!("Unexpected token: {:#?}", other);
                    }
                }
            }
        }
        return Ok(tokens);
    }
    pub fn parse(&self, input: String) -> Result<node::Node, String> {
        let _tokens = self.tokenize(input);
        let value: node::Node = node::Node::Null;
        return Ok(value);
    }
}
