use crate::tokenizer;

#[derive(Debug)]
pub struct Refiner {}
impl Refiner {
    pub fn new() -> Refiner {
        return Refiner {};
    }
    pub fn refine_tokens(
        &self,
        _input: Vec<tokenizer::Token>,
    ) -> Result<Vec<tokenizer::Token>, String> {
        let tokens: Vec<tokenizer::Token> = Vec::new();
        return Ok(tokens);
    }
}
