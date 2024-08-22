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

pub fn parse_object(tokens: &mut Iter<token::Token>) -> Result<HashMap<String, Node>, String> {
    let mut object = HashMap::new();
    let mut is_key = true;
    let mut current_key: Option<&str> = None;

    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBraceOpen => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::Object(parse_object(tokens)?));
                }
            }
            token::TokenKind::Colon => {
                is_key = false;
            }
            token::TokenKind::BracketOpen => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::Array(parse_array(tokens)?));
                    current_key = None;
                }
            }
            token::TokenKind::String => {
                if is_key == true {
                    current_key = Some(curr_token.value.as_str());
                } else if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::String(curr_token.value.clone()));
                    current_key = None;
                }
            }
            token::TokenKind::Number => {
                if let Some(key) = current_key {
                    object.insert(
                        key.to_string(),
                        Node::Number(match curr_token.value.clone().parse::<f64>() {
                            Ok(safe_value) => safe_value,
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }),
                    );
                    current_key = None;
                }
            }
            token::TokenKind::True => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::True);
                    current_key = None;
                }
            }
            token::TokenKind::False => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::False);
                    current_key = None;
                }
            }
            token::TokenKind::Null => {
                if let Some(key) = current_key {
                    object.insert(key.to_string(), Node::Null);
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
    return Ok(object);
}
pub fn parse_array(tokens: &mut Iter<token::Token>) -> Result<Vec<Node>, String> {
    let mut array: Vec<Node> = Vec::<Node>::new();
    while let Some(curr_token) = tokens.next() {
        match curr_token.kind {
            token::TokenKind::CurlyBraceOpen => {
                array.push(Node::Object(parse_object(tokens)?));
            }
            token::TokenKind::BracketOpen => {
                array.push(Node::Array(parse_array(tokens)?));
            }
            token::TokenKind::String => {
                array.push(Node::String(curr_token.value.clone()));
            }
            token::TokenKind::Number => match curr_token.value.clone().parse::<f64>() {
                Ok(safe_value) => {
                    array.push(Node::Number(safe_value));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            token::TokenKind::True => {
                array.push(Node::True);
            }
            token::TokenKind::False => {
                array.push(Node::False);
            }
            token::TokenKind::Null => {
                array.push(Node::Null);
            }
            token::TokenKind::BracketClose => {
                break;
            }
            token::TokenKind::CurlyBraceClose
            | token::TokenKind::Comma
            | token::TokenKind::Colon => {}
        }
    }
    return Ok(array);
}
