use crate::tokenizer;
use std::{collections::HashMap, iter::Peekable, slice::Iter};

#[derive(Debug)]
pub enum ASTNode {
    Table(HashMap<String, ASTNode>),
    Array(Vec<ASTNode>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug)]
pub struct ASTBuilder {}
impl ASTBuilder {
    pub fn new() -> ASTBuilder {
        ASTBuilder {}
    }
    pub fn parse_table(
        &self,
        tokens: &mut Peekable<Iter<tokenizer::Token>>,
    ) -> HashMap<String, ASTNode> {
        let table: HashMap<String, ASTNode> = HashMap::new();
        // let mut is_key = true;
        // let mut current_key: Option<&str> = None;

        while let Some(curr_token) = tokens.next() {
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

    pub fn parse_array(&self, tokens: &mut Peekable<Iter<tokenizer::Token>>) -> Vec<ASTNode> {
        let array: Vec<ASTNode> = Vec::new();
        while let Some(curr_token) = tokens.next() {
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
