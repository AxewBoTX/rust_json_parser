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
            // ----- every value that needs to be refined is a value in a key-value pair, including
            // the things in a array, because the array itself will be a value in a key-value pair.
            // So, instead of checking for key-value pair inside the match statement, check for it
            // outside, it would make things easily -----
            if current_token.kind == tokenizer::TokenKind::NonQuoteString {}
            match current_token.kind {
                // non-quote string
                tokenizer::TokenKind::NonQuoteString => {
                    if self
                        .list
                        .peek_before(1)
                        .is_some_and(|prev_token| prev_token.kind == tokenizer::TokenKind::EqualTo)
                    {
                        // value in a key-value pair
                        if [String::from("true"), String::from("false")]
                            .contains(&current_token.value)
                        {
                            // boolean
                            tokens.push(tokenizer::Token::new(
                                tokenizer::TokenKind::Boolean,
                                current_token.value.clone(),
                            ));
                            let _ = self.list.current();
                        } else if utils::is_integer(&current_token.value) {
                            // number
                            let mut value = String::new();
                            while let Some(next_token) = self.list.peek() {
                                if next_token.kind == tokenizer::TokenKind::Dot
                                    || next_token.kind == tokenizer::TokenKind::NonQuoteString
                                        && utils::is_integer(&next_token.value)
                                {
                                    value.push_str(&next_token.value);
                                    let _ = self.list.current();
                                } else {
                                    break;
                                }
                            }
                            if utils::is_integer(&value) {
                                // integer
                                match value.parse::<i64>() {
                                    Ok(safe_value) => {
                                        tokens.push(tokenizer::Token::new(
                                            tokenizer::TokenKind::Integer,
                                            safe_value.to_string(),
                                        ));
                                    }
                                    Err(e) => {
                                        return Err(e.to_string());
                                    }
                                }
                            } else if utils::is_float(&value) {
                                // float
                                match value.parse::<f64>() {
                                    Ok(safe_value) => {
                                        tokens.push(tokenizer::Token::new(
                                            tokenizer::TokenKind::Float,
                                            safe_value.to_string(),
                                        ));
                                    }
                                    Err(e) => {
                                        return Err(e.to_string());
                                    }
                                }
                            } else {
                                return Err(format!("Unexpected token: {:#?}", &value));
                            }
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
