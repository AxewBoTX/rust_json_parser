use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Table(HashMap<String, Node>),
    Array(Vec<Node>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}
