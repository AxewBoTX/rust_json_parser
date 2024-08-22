use crate::{tokenizer, utils};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ASTNode {
    Table(HashMap<String, ASTNode>),
    Array(Vec<ASTNode>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ASTBuilder {
    pub list: utils::IteratorList<tokenizer::Token>,
}
impl ASTBuilder {
    pub fn new(list: &Vec<tokenizer::Token>) -> ASTBuilder {
        return ASTBuilder {
            list: utils::IteratorList::new(list),
        };
    }
    // main parsing function
    pub fn parse(&mut self) -> Result<HashMap<String, ASTNode>, String> {
        let mut main_table: HashMap<String, ASTNode> = HashMap::new();

        let mut is_key = true;
        let mut current_key: Option<String> = None;

        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {
                    if let Some(key) = &current_key {
                        main_table.insert(key.to_string(), ASTNode::Table(self.parse_table()?));
                    }
                }
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => {
                    if is_key == true {
                        let mut table_heading: Option<String> = None;
                        while let Some(curr_token) = self.list.current() {
                            match curr_token.kind {
                                tokenizer::TokenKind::QuoteString
                                | tokenizer::TokenKind::NonQuoteString => {
                                    table_heading = Some(curr_token.value.clone());
                                }
                                tokenizer::TokenKind::BracketClose => {
                                    if let Some(heading) = &table_heading {
                                        main_table.insert(
                                            heading.to_string(),
                                            ASTNode::Table(self.parse_table()?),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    } else if let Some(key) = &current_key {
                        main_table.insert(key.to_string(), ASTNode::Array(self.parse_array()?));
                    }
                }
                tokenizer::TokenKind::BracketClose => {}
                tokenizer::TokenKind::EqualTo => {
                    is_key = false;
                }
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {
                    is_key = true;
                }
                tokenizer::TokenKind::QuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone());
                    } else if let Some(key) = &current_key {
                        main_table.insert(key.to_string(), ASTNode::String(token.value.clone()));
                    }
                }
                tokenizer::TokenKind::NonQuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone());
                    } else if let Some(key) = &current_key {
                        main_table.insert(key.to_string(), ASTNode::String(token.value.clone()));
                    }
                }
                tokenizer::TokenKind::Boolean => {
                    if let Some(key) = &current_key {
                        if token.value == String::from("true") {
                            main_table.insert(key.to_string(), ASTNode::Boolean(true));
                        } else if token.value == String::from("false") {
                            main_table.insert(key.to_string(), ASTNode::Boolean(false));
                        }
                    }
                }
                tokenizer::TokenKind::Integer => {
                    if let Some(key) = &current_key {
                        match token.value.parse::<i64>() {
                            Ok(safe_value) => {
                                main_table.insert(key.to_string(), ASTNode::Integer(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                tokenizer::TokenKind::Float => {
                    if let Some(key) = &current_key {
                        match token.value.parse::<f64>() {
                            Ok(safe_value) => {
                                main_table.insert(key.to_string(), ASTNode::Float(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                tokenizer::TokenKind::NewLine => {
                    is_key = true;
                }
            }
        }

        return Ok(main_table);
    }

    // parse `Array`
    pub fn parse_array(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut array: Vec<ASTNode> = Vec::new();
        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {
                    array.push(ASTNode::Table(self.parse_table()?))
                }
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => {
                    array.push(ASTNode::Array(self.parse_array()?));
                }
                tokenizer::TokenKind::BracketClose => {
                    break;
                }
                tokenizer::TokenKind::EqualTo => {}
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {}
                tokenizer::TokenKind::QuoteString => {
                    array.push(ASTNode::String(token.value.clone()));
                }
                tokenizer::TokenKind::NonQuoteString => {
                    array.push(ASTNode::String(token.value.clone()));
                }
                tokenizer::TokenKind::Boolean => {
                    if token.value == String::from("true") {
                        array.push(ASTNode::Boolean(true));
                    } else if token.value == String::from("false") {
                        array.push(ASTNode::Boolean(false));
                    }
                }
                tokenizer::TokenKind::Integer => match token.value.parse::<i64>() {
                    Ok(safe_value) => {
                        array.push(ASTNode::Integer(safe_value));
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                tokenizer::TokenKind::Float => match token.value.parse::<f64>() {
                    Ok(safe_value) => {
                        array.push(ASTNode::Float(safe_value));
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                tokenizer::TokenKind::NewLine => {}
            }
        }
        return Ok(array);
    }

    // parse `Table`
    pub fn parse_table(&mut self) -> Result<HashMap<String, ASTNode>, String> {
        let mut table: HashMap<String, ASTNode> = HashMap::new();

        let mut is_key = true;
        let mut current_key: Option<String> = None;

        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {
                    if let Some(key) = &current_key {
                        table.insert(key.to_string(), ASTNode::Table(self.parse_table()?));
                    }
                }
                tokenizer::TokenKind::CurlyBracketClose => {
                    break;
                }
                tokenizer::TokenKind::BracketOpen => {
                    if is_key == true {
                    } else if let Some(key) = &current_key {
                        table.insert(key.to_string(), ASTNode::Array(self.parse_array()?));
                    }
                }
                tokenizer::TokenKind::BracketClose => {}
                tokenizer::TokenKind::EqualTo => {
                    is_key = false;
                }
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {
                    is_key = true;
                }
                tokenizer::TokenKind::QuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone());
                    } else if let Some(key) = &current_key {
                        table.insert(key.to_string(), ASTNode::String(token.value.clone()));
                    }
                }
                tokenizer::TokenKind::NonQuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone());
                    } else if let Some(key) = &current_key {
                        table.insert(key.to_string(), ASTNode::String(token.value.clone()));
                    }
                }
                tokenizer::TokenKind::Boolean => {
                    if let Some(key) = &current_key {
                        if token.value == String::from("true") {
                            table.insert(key.to_string(), ASTNode::Boolean(true));
                        } else if token.value == String::from("false") {
                            table.insert(key.to_string(), ASTNode::Boolean(false));
                        }
                    }
                }
                tokenizer::TokenKind::Integer => {
                    if let Some(key) = &current_key {
                        match token.value.parse::<i64>() {
                            Ok(safe_value) => {
                                table.insert(key.to_string(), ASTNode::Integer(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                tokenizer::TokenKind::Float => {
                    if let Some(key) = &current_key {
                        match token.value.parse::<f64>() {
                            Ok(safe_value) => {
                                table.insert(key.to_string(), ASTNode::Float(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                tokenizer::TokenKind::NewLine => {
                    is_key = true;
                }
            }
            if is_key == true
                && self
                    .list
                    .peek()
                    .is_some_and(|next_token| next_token.kind == tokenizer::TokenKind::BracketOpen)
            {
                break;
            }
        }

        return Ok(table);
    }
}
