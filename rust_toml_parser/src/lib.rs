pub mod ast_builder;
pub mod refiner;
pub mod tokenizer;
pub mod utils;

#[derive(Debug)]
pub struct TomlParser {}

impl TomlParser {
    pub fn new() -> TomlParser {
        return TomlParser {};
    }
    pub fn tokenize(&self, input: String) -> Result<Vec<tokenizer::Token>, String> {
        let mut tokens: Vec<tokenizer::Token> = Vec::new();
        let mut toml_tokenizer = tokenizer::Tokenizer::new(&input.chars().collect::<Vec<char>>());

        while let Some(character) = toml_tokenizer.list.peek() {
            match character {
                '#' => {
                    while Some('\n') != toml_tokenizer.list.peek() {
                        toml_tokenizer.list.current();
                    }
                    if Some('\n') == toml_tokenizer.list.peek() {
                        toml_tokenizer.list.current();
                    }
                }
                '{' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::CurlyBracketOpen,
                        "{".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '}' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::CurlyBracketOpen,
                        "}".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '[' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::BracketOpen,
                        "[".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                ']' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::BracketClose,
                        "]".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                ',' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::Comma,
                        ",".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '.' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::Dot,
                        ".".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '=' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::EqualTo,
                        "=".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '"' => {
                    toml_tokenizer.list.current();
                    tokens.push(toml_tokenizer.tokenize_quote_string());
                }
                '\n' => {
                    tokens.push(tokenizer::Token::new(
                        tokenizer::TokenKind::NewLine,
                        "\n".to_string(),
                    ));
                    toml_tokenizer.list.current();
                }
                '\0' => {
                    break;
                }
                other => {
                    if other.is_whitespace() {
                        toml_tokenizer.list.current();
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
        return Ok(tokens);
    }
    pub fn parse(&self, input: Vec<tokenizer::Token>) -> Result<ast_builder::ASTNode, String> {
        let mut toml_refiner = refiner::Refiner::new(&input);
        match toml_refiner.refine_tokens() {
            Ok(safe_value) => {
                println!("{:#?}", safe_value);
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
