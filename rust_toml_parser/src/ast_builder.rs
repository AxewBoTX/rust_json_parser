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
        todo!();
    }
    // parse table heading
    pub fn parse_table_heading(&mut self) -> Result<(String, HashMap<String, ASTNode>), String> {
        todo!();
    }
    // parse `Table`
    pub fn parse_table(&mut self) -> Result<HashMap<String, ASTNode>, String> {
        todo!();
    }

    pub fn parse_array(&mut self) -> Result<Vec<ASTNode>, String> {
        todo!();
    }
}
