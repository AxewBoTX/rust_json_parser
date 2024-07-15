use crate::node;
use crate::token;
use std::{collections::HashMap, slice::Iter};

#[derive(Debug)]
pub enum Node {
    Object(HashMap<String, Node>),
    Array(Vec<Node>),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn parse_object(tokens: &mut Iter<token::Token>) -> HashMap<String, Node> {
    let mut object = HashMap::new();
    let mut is_key = true;
    let mut current_key: Option<&str> = None;

    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBraceOpen => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), node::Node::Object(parse_object(tokens)));
                }
            }
            token::TokenKind::Colon => {
                is_key = false;
            }
            token::TokenKind::BracketOpen => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), node::Node::Array(parse_array(tokens)));
                    current_key = None;
                }
            }
            token::TokenKind::String => {
                if is_key == true {
                    current_key = Some(curr_token.value.as_str());
                } else if let Some(key) = current_key {
                    object.insert(
                        key.to_string(),
                        node::Node::String(curr_token.value.clone()),
                    );
                    current_key = None;
                }
            }
            token::TokenKind::Number => {
                if let Some(key) = current_key {
                    object.insert(
                        key.to_string(),
                        node::Node::Number(curr_token.value.clone().parse::<f64>().unwrap()),
                    );
                    current_key = None;
                }
            }
            token::TokenKind::True => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), node::Node::True);
                    current_key = None;
                }
            }
            token::TokenKind::False => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), node::Node::False);
                    current_key = None;
                }
            }
            token::TokenKind::Null => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), node::Node::Null);
                    current_key = None;
                }
            }
            token::TokenKind::Comma => {
                is_key = true;
            }
            token::TokenKind::CurlyBraceClose => {
                break;
            }
            // no need to handle them here
            token::TokenKind::BracketClose => {}
        }
    }
    return object;
}
pub fn parse_array(tokens: &mut Iter<token::Token>) -> Vec<Node> {
    let mut array: Vec<Node> = Vec::<Node>::new();
    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBraceOpen => {
                array.push(node::Node::Object(parse_object(tokens)));
            }
            token::TokenKind::BracketOpen => {
                array.push(node::Node::Array(parse_array(tokens)));
            }
            token::TokenKind::String => {
                array.push(node::Node::String(curr_token.value.clone()));
            }
            token::TokenKind::Number => {
                array.push(node::Node::Number(
                    curr_token.value.clone().parse::<f64>().unwrap(),
                ));
            }
            token::TokenKind::True => {
                array.push(node::Node::True);
            }
            token::TokenKind::False => {
                array.push(node::Node::False);
            }
            token::TokenKind::Null => {
                array.push(node::Node::Null);
            }
            token::TokenKind::BracketClose => {
                break;
            }
            token::TokenKind::CurlyBraceClose
            | token::TokenKind::Comma
            | token::TokenKind::Colon => {}
        }
    }
    return array;
}
