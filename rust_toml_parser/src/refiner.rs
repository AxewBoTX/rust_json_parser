use crate::{tokenizer, utils};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Refiner {
    pub list: utils::IteratorList<tokenizer::Token>,
}
impl Refiner {
    pub fn new(list: &Vec<tokenizer::Token>) -> Refiner {
        return Refiner {
            list: utils::IteratorList::new(list),
        };
    }
    pub fn refine_tokens(&mut self) -> Result<Vec<tokenizer::Token>, String> {
        let mut tokens: Vec<tokenizer::Token> = Vec::new();
        while let Some(current_token) = self.list.current() {
            match current_token.kind {
                _ => {
                    tokens.push(current_token);
                }
            }
        }
        return Ok(tokens);
    }
}
