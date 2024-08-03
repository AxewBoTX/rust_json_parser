pub mod ast_builder;
pub mod refiner;
pub mod tokenizer;
pub mod utils;

use utils::TomlIterator;

#[derive(Debug)]
pub struct TomlParser {}

impl TomlParser {
    pub fn new() -> TomlParser {
        return TomlParser {};
    }
    pub fn tokenize(&self, input: String) -> Result<Vec<tokenizer::Token>, String> {
        let mut tokens: Vec<tokenizer::Token> = Vec::new();
        let mut toml_tokenizer = tokenizer::Tokenizer::new(&input.chars().collect::<Vec<char>>());

        while let Some(character) = toml_tokenizer.peek() {
            match character {
                '#' => {
                    // ----- this implementation can be improved ----
                    while Some('\n') != toml_tokenizer.peek() {
                        toml_tokenizer.next();
                    }
                    if Some('\n') == toml_tokenizer.peek() {
                        toml_tokenizer.next();
                    }
                }
                '{' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::CurlyBracketOpen,
                        "{".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '}' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::CurlyBracketOpen,
                        "}".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '[' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::BracketOpen,
                        "[".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                ']' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::BracketClose,
                        "]".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                ',' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::Comma,
                        ",".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '.' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::Dot,
                        ".".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '=' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::EqualTo,
                        "=".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '"' => {
                    toml_tokenizer.next();
                    tokens.push(toml_tokenizer.tokenize_quote_string());
                }
                '\n' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::NewLine,
                        "\n".to_string(),
                    ));
                    toml_tokenizer.next();
                }
                '\0' => {
                    break;
                }
                other => {
                    if other.is_whitespace() {
                        toml_tokenizer.next();
                    } else if other.is_alphanumeric() || ['_', '-'].contains(&other) {
                        match toml_tokenizer.tokenize_nonquote_string() {
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
        println!("{:#?}", tokens);
        return Ok(tokens);
    }
    pub fn parse(&self, input: Vec<tokenizer::Token>) -> Result<ast_builder::ASTNode, String> {
        let toml_refiner = refiner::Refiner::new();
        match toml_refiner.refine_tokens(input) {
            Ok(safe_value) => {
                let mut tokens = safe_value.iter().peekable();
                let toml_ast_builder = ast_builder::ASTBuilder::new();
                return Ok(ast_builder::ASTNode::Table(
                    toml_ast_builder.parse_table(&mut tokens),
                ));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
