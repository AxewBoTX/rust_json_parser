pub mod ast_builder;
pub mod token_refiner;
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
        let tokenizer_input = input.chars().collect::<Vec<char>>();
        match tokenizer_input.len() {
            0 => {
                return Err("empty input file".to_string());
            }
            _ => {}
        }
        let mut toml_tokenizer = tokenizer::Tokenizer::new(&tokenizer_input);

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
                        tokenizer::TokenKind::CurlyBracketClose,
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
                        tokens.push(toml_tokenizer.tokenize_nonquote_string()?);
                    } else {
                        return Err(format!("Unexpected character: {:#?}", other));
                    }
                }
            }
        }
        return Ok(tokens);
    }
    pub fn parse(&self, input: Vec<tokenizer::Token>) -> Result<ast_builder::ASTNode, String> {
        match input.len() {
            0 => {
                return Err("no syntax tokens detected".to_string());
            }
            _ => {}
        }
        let mut toml_token_refiner = token_refiner::TokenRefiner::new(&input);
        let refined_tokens = toml_token_refiner.refine_tokens()?;
        let mut toml_ast_builder = ast_builder::ASTBuilder::new(&refined_tokens);
        let main_table = toml_ast_builder.parse()?;
        return Ok(ast_builder::ASTNode::Table(main_table));
    }
}
