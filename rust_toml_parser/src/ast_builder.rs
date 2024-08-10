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

        let mut current_key: Option<String> = None;
        let mut is_key = true;

        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::NonQuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone());
                    } else {
                        return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                    }
                }
                tokenizer::TokenKind::EqualTo => {
                    is_key = false;
                }
                tokenizer::TokenKind::QuoteString => {
                    if is_key == true {
                        current_key = Some(token.value.clone())
                    } else if let Some(key) = &current_key {
                        main_table.insert(key.to_string(), ASTNode::String(token.value.clone()));
                    }
                }
                tokenizer::TokenKind::Boolean => {
                    if let Some(key) = &current_key {
                        if token.value.to_lowercase() == "true".to_string() {
                            main_table.insert(key.to_string(), ASTNode::Boolean(true));
                        } else if token.value.to_lowercase() == "false".to_string() {
                            main_table.insert(key.to_string(), ASTNode::Boolean(false));
                        } else {
                            return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
                        }
                    }
                }
                tokenizer::TokenKind::Integer => {
                    if is_key == true {
                        current_key = Some(token.value.clone())
                    } else if let Some(key) = &current_key {
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
                    if is_key == true {
                        current_key = Some(token.value.clone())
                    } else if let Some(key) = &current_key {
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
                tokenizer::TokenKind::CurlyBracketOpen => {}
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => {
                    if is_key == true {
                        match self.parse_table_heading() {
                            Ok((safe_key, safe_value)) => {
                                main_table.insert(safe_key.to_string(), ASTNode::Table(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    } else if let Some(key) = &current_key {
                        match self.parse_array() {
                            Ok(safe_value) => {
                                main_table.insert(key.to_string(), ASTNode::Array(safe_value));
                            }
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                tokenizer::TokenKind::BracketClose => {}
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {}
            }
        }
        return Ok(main_table);
    }
    // parse table heading
    pub fn parse_table_heading(&mut self) -> Result<(String, HashMap<String, ASTNode>), String> {
        let mut table: HashMap<String, ASTNode> = HashMap::new();
        let mut table_heading: String = String::new();
        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::NonQuoteString | tokenizer::TokenKind::QuoteString => {
                    table_heading = token.value.to_string();
                }
                tokenizer::TokenKind::Dot => match self.parse_table_heading() {
                    Ok((safe_heading, safe_value)) => {
                        table.insert(safe_heading, ASTNode::Table(safe_value));
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                tokenizer::TokenKind::BracketClose => {}
                _ => {}
            }
        }
        return Ok((table_heading, table));
    }
    // parse `Table`
    pub fn parse_table(&mut self) -> HashMap<String, ASTNode> {
        let table: HashMap<String, ASTNode> = HashMap::new();

        while let Some(curr_token) = self.list.current() {
            match curr_token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {}
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => {}
                tokenizer::TokenKind::BracketClose => {}
                tokenizer::TokenKind::EqualTo => {}
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {}
                tokenizer::TokenKind::QuoteString => {}
                tokenizer::TokenKind::NonQuoteString => {}
                tokenizer::TokenKind::NewLine => {}
                tokenizer::TokenKind::Boolean => {}
                tokenizer::TokenKind::Integer => {}
                tokenizer::TokenKind::Float => {}
            }
        }
        return table;
    }

    fn parse_array(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut array: Vec<ASTNode> = Vec::new();

        while let Some(token) = self.list.current() {
            match token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {}
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => match self.parse_array() {
                    Ok(safe_value) => array.push(ASTNode::Array(safe_value)),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                },
                tokenizer::TokenKind::BracketClose => {
                    break;
                }
                tokenizer::TokenKind::EqualTo => {
                    return Err(format!("unexpected token: {:#?}", token).to_string());
                }
                tokenizer::TokenKind::Dot => {
                    return Err(format!("unexpected token: {:#?}", token).to_string());
                }
                tokenizer::TokenKind::Comma => {}
                tokenizer::TokenKind::QuoteString => {
                    array.push(ASTNode::String(token.value.to_string()))
                }
                tokenizer::TokenKind::NonQuoteString => {
                    return Err(format!("unexpected token: {:#?}", token).to_string());
                }
                tokenizer::TokenKind::NewLine => {}
                tokenizer::TokenKind::Boolean => {
                    if token.value.to_lowercase() == "true".to_string() {
                        array.push(ASTNode::Boolean(true));
                    } else if token.value.to_lowercase() == "false".to_string() {
                        array.push(ASTNode::Boolean(false));
                    } else {
                        return Err("something went terribly wrong! (most probably a language problem, not a code problem)".to_string());
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
            }
        }
        return Ok(array);
    }
}
