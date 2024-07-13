use crate::token;
use std::{collections::HashMap, iter::Peekable, slice::Iter};

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

pub fn parse_object(tokens: &mut Peekable<Iter<token::Token>>) {
    todo!("{:#?}", tokens);
}
pub fn parse_array(tokens: &mut Peekable<Iter<token::Token>>) {
    todo!("{:#?}", tokens);
}
