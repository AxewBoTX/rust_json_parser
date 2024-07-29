use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Table(HashMap<String, Node>),
    Array(Vec<Node>),
    Integer,
    Float,
    Boolean,
    String,
}
