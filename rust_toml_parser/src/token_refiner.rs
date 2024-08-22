use crate::{tokenizer, utils};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenRefiner {
    pub list: utils::IteratorList<tokenizer::Token>,
}
impl TokenRefiner {
    pub fn new(list: &Vec<tokenizer::Token>) -> TokenRefiner {
        return TokenRefiner {
            list: utils::IteratorList::new(list),
        };
    }
    // main refining function
    pub fn refine_tokens(&mut self) -> Result<Vec<tokenizer::Token>, String> {
        let mut tokens: Vec<tokenizer::Token> = Vec::new();
        while let Some(current_token) = self.list.peek() {
            if [
                tokenizer::TokenKind::NonQuoteString,
                tokenizer::TokenKind::BracketOpen,
            ]
            .contains(&current_token.kind)
                && self
                    .list
                    .peek_before(1)
                    .is_some_and(|prev_token| prev_token.kind == tokenizer::TokenKind::EqualTo)
            {
                match current_token.kind {
                    // non-quote string
                    tokenizer::TokenKind::NonQuoteString => {
                        tokens.extend(self.refine_non_quote_string()?)
                    }

                    // array
                    tokenizer::TokenKind::BracketOpen => tokens.extend(self.refine_array()?),
                    _ => {
                        tokens.push(current_token.clone());
                        match self.list.current() {
                            Some(_) => {}
                            None => {
                                return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                            }
                        }
                    }
                }
            } else {
                tokens.push(current_token.clone());
                match self.list.current() {
                    Some(_) => {}
                    None => {
                        return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                    }
                }
            }
        }
        return Ok(tokens);
    }
    // refine `NonQuoteString` tokens
    fn refine_non_quote_string(&mut self) -> Result<Vec<tokenizer::Token>, String> {
        match self.list.peek() {
            Some(current_token) => {
                // inside an inline object
                if self.list.peek_before(1).is_some_and(|prev_token| {
                    prev_token.kind == tokenizer::TokenKind::CurlyBracketOpen
                }) {
                    let mut tokens: Vec<tokenizer::Token> = Vec::new();
                    while let Some(token) = self.list.peek() {
                        match token.kind {
                            tokenizer::TokenKind::EqualTo => {
                                match self.list.current() {
                                    Some(safe_value) => tokens.push(safe_value),
                                    None => {
                                        return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                                    }
                                }
                                break;
                            }
                            _ => match self.list.current() {
                                Some(safe_value) => tokens.push(safe_value),
                                None => {
                                    return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                                }
                            },
                        }
                    }
                    return Ok(tokens);
                } else {
                    // value in a key-value pair
                    if [String::from("true"), String::from("false")].contains(&current_token.value)
                    {
                        // boolean
                        match self.list.current() {
                            Some(_) => {}
                            None => {
                                return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                            }
                        }
                        return Ok(vec![tokenizer::Token::new(
                            tokenizer::TokenKind::Boolean,
                            current_token.value.clone(),
                        )]);
                    } else if utils::is_integer(&current_token.value) {
                        return Ok(vec![self.refine_number()?]);
                    } else {
                        match self.list.current() {
                            Some(_) => {}
                            None => {
                                return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                            }
                        }
                        return Ok(vec![current_token.clone()]);
                    }
                }
            }
            None => {
                return Err("failed to refine non-quote string".to_string());
            }
        }
    }
    // refine array
    fn refine_array(&mut self) -> Result<Vec<tokenizer::Token>, String> {
        let mut tokens: Vec<tokenizer::Token> = Vec::new();
        match self.list.current() {
            Some(open_bracket) => {
                tokens.push(open_bracket);
            }
            None => return Err("failed to refine arrray".to_string()),
        }
        while let Some(current_token) = self.list.peek() {
            match current_token.kind {
                // non-quote string
                tokenizer::TokenKind::NonQuoteString => {
                    tokens.extend(self.refine_non_quote_string()?)
                }
                // array
                tokenizer::TokenKind::BracketOpen => tokens.extend(self.refine_array()?),
                tokenizer::TokenKind::BracketClose => {
                    tokens.push(current_token.clone());
                    match self.list.current() {
                        Some(_) => {}
                        None => {
                            return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                        }
                    }
                    break;
                }
                _ => {
                    tokens.push(current_token.clone());
                    match self.list.current() {
                        Some(_) => {}
                        None => {
                            return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                        }
                    }
                }
            }
        }
        return Ok(tokens);
    }
    // refine number
    fn refine_number(&mut self) -> Result<tokenizer::Token, String> {
        let mut value = String::new();
        while let Some(next_token) = self.list.peek() {
            if next_token.kind == tokenizer::TokenKind::Dot
                || next_token.kind == tokenizer::TokenKind::NonQuoteString
                    && utils::is_integer(&next_token.value)
            {
                value.push_str(&next_token.value);
                match self.list.current() {
                    Some(_) => {}
                    None => {
                        return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                    }
                }
            } else {
                break;
            }
        }
        if utils::is_integer(&value) {
            // integer
            match value.parse::<i64>() {
                Ok(safe_value) => {
                    return Ok(tokenizer::Token::new(
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
                    return Ok(tokenizer::Token::new(
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
    }
}
