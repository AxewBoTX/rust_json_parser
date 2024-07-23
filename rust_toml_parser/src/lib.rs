pub mod node;
pub mod token;

#[derive(Debug)]
pub struct TomlParser {}

impl TomlParser {
    pub fn new() -> TomlParser {
        return TomlParser {};
    }
    pub fn tokenize(&self, _input: String) -> Result<Vec<token::Token>, String> {
        let tokens: Vec<token::Token> = Vec::new();
        return Ok(tokens);
    }
    pub fn parse(&self, input: String) -> Result<node::Node, String> {
        let _tokens = self.tokenize(input);
        let value: node::Node = node::Node::Null;
        return Ok(value);
    }
}
