use crate::token;
use std::{collections::HashMap, iter::Peekable, slice::Iter};

#[derive(Debug)]
pub enum Node {
    Table(HashMap<String, Node>),
    Array(Vec<Node>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

pub fn parse_table(tokens: &mut Peekable<Iter<token::Token>>) -> HashMap<String, Node> {
    let table: HashMap<String, Node> = HashMap::new();
    // let mut is_key = true;
    // let mut current_key: Option<&str> = None;

    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBracketOpen => {}
            token::TokenKind::CurlyBracketClose => {}
            token::TokenKind::BracketOpen => {}
            token::TokenKind::BracketClose => {}
            token::TokenKind::EqualTo => {
                // is_key = false;
            }
            token::TokenKind::Dot => {}
            token::TokenKind::Comma => {}
            token::TokenKind::QuoteString => {}
            token::TokenKind::NonQuoteString => {
                // if is_key == true {
                //     current_key = Some(curr_token.value.as_str());
                // } else if let Some(key) = current_key {
                //     table.insert(key.to_string(), Node::String(curr_token.value.clone()));
                //     current_key = None;
                // }
            }
            token::TokenKind::NewLine => {
                // is_key = false;
            }
        }
    }
    return table;
}

pub fn parse_array(tokens: &mut Peekable<Iter<token::Token>>) -> Vec<Node> {
    let array: Vec<Node> = Vec::new();
    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBracketOpen => {}
            token::TokenKind::CurlyBracketClose => {}
            token::TokenKind::BracketOpen => {}
            token::TokenKind::BracketClose => {}
            token::TokenKind::EqualTo => {}
            token::TokenKind::Dot => {}
            token::TokenKind::Comma => {}
            token::TokenKind::QuoteString => {}
            token::TokenKind::NonQuoteString => {}
            token::TokenKind::NewLine => {}
        }
    }
    return array;
}
