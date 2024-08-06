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
        while let Some(current_token) = self.list.peek() {
            match current_token.kind {
                tokenizer::TokenKind::NonQuoteString => {
                    // value in a key-value pair
                    if self.list.peek_before(1).is_some_and(|prev_token| {
                        println!("Equal To");
                        prev_token.kind == tokenizer::TokenKind::EqualTo
                    }) {
                        // boolean
                        if [String::from("true"), String::from("false")]
                            .contains(&current_token.value)
                        {
                            tokens.push(tokenizer::Token::new(
                                tokenizer::TokenKind::Boolean,
                                current_token.value.clone(),
                            ));
                            let _ = self.list.current();
                        } else {
                            tokens.push(current_token.clone());
                            let _ = self.list.current();
                        }
                    } else {
                        tokens.push(current_token.clone());
                        let _ = self.list.current();
                    }
                }
                _ => {
                    tokens.push(current_token.clone());
                    let _ = self.list.current();
                }
            }
        }
        return Ok(tokens);
    }
}
