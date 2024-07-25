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
                    while input.peek() != Some(&'\n') {
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
                // '-' | '0'..='9' => match token::Token::tokenize_number(&mut input) {
                //     Ok(safe_value) => {
                //         tokens.push(safe_value);
                //     }
                //     Err(e) => {
                //         return Err(e.to_string());
                //     }
                // },
                // 't' => {
                //     // tokens.push(token::Token::tokenize_true(&mut input));
                // }
                // 'f' => {
                //     // tokens.push(token::Token::tokenize_false(&mut input));
                // }
                '\0' => {
                    break;
                }
                other => {
                    if other.is_whitespace() {
                        input.next();
                    }
                    // if other.is_alphabetic() {
                    //     // tokens.push(token::Token::tokenize_nonquote_string(&mut input));
                    // } else {
                    //     // panic!("Unexpected token: {:#?}", other);
                    // }
                }
            }
        }
        println!("Tokens: {:#?}", tokens);
        return Ok(tokens);
    }
    pub fn parse(&self, input: String) -> Result<node::Node, String> {
        let _tokens = self.tokenize(input);
        let value: node::Node = node::Node::Null;
        return Ok(value);
    }
}
