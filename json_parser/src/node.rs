use std::collections::HashMap;

pub enum Node {
    Object(HashMap<String, Node>),
    Array(Vec<Node>),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn parse_object() {}
pub fn parse_array() {}
pub fn parse_string() {}
pub fn parse_number() {}
