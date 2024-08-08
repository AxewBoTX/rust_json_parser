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
    pub fn parse(&mut self) -> HashMap<String, ASTNode> {
        let main_table: HashMap<String, ASTNode> = HashMap::new();
        return main_table;
    }
    pub fn parse_table(&mut self) -> HashMap<String, ASTNode> {
        let table: HashMap<String, ASTNode> = HashMap::new();
        // let mut is_key = true;
        // let mut current_key: Option<&str> = None;

        while let Some(curr_token) = self.list.current() {
            match curr_token.kind {
                tokenizer::TokenKind::CurlyBracketOpen => {}
                tokenizer::TokenKind::CurlyBracketClose => {}
                tokenizer::TokenKind::BracketOpen => {}
                tokenizer::TokenKind::BracketClose => {}
                tokenizer::TokenKind::EqualTo => {
                    // is_key = false;
                }
                tokenizer::TokenKind::Dot => {}
                tokenizer::TokenKind::Comma => {}
                tokenizer::TokenKind::QuoteString => {}
                tokenizer::TokenKind::NonQuoteString => {
                    // if is_key == true {
                    //     current_key = Some(curr_token.value.as_str());
                    // } else if let Some(key) = current_key {
                    //     table.insert(key.to_string(), ASTNode::String(curr_token.value.clone()));
                    //     current_key = None;
                    // }
                }
                tokenizer::TokenKind::NewLine => {
                    // is_key = false;
                }
                tokenizer::TokenKind::Boolean => {}
                tokenizer::TokenKind::Integer => {}
                tokenizer::TokenKind::Float => {}
            }
        }
        return table;
    }

    pub fn parse_array(&mut self) -> Vec<ASTNode> {
        let array: Vec<ASTNode> = Vec::new();
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
        return array;
    }
}
