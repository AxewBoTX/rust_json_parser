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
                        match token::Token::tokenize_nonquote_string(&mut input) {
                            Ok(safe_value) => {
                                tokens.push(safe_value);
                            }
                            Err(e) => {
                                panic!("{}", e.to_string());
                            }
                        }
                    } else {
                        panic!("Unexpected character: {:#?}", other);
                    }
                }
            }
        }
        return Ok(tokens);
    }
    pub fn parse(&self, input: String) -> Result<node::Node, String> {
        match self.tokenize(input) {
            Ok(tokens) => {
                let mut tokens = tokens.iter();
                while let Some(_token) = tokens.next() {
                    // println!("Tokens: {:#?}", token);
                }
                return Err("".to_string());
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
