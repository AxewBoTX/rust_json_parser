use crate::utils::{self, TomlIterator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    CurlyBracketOpen,
    CurlyBracketClose,
    BracketOpen,
    BracketClose,
    EqualTo,
    Dot,
    Comma,
    QuoteString,
    NonQuoteString,
    NewLine,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}
impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        return Token { kind, value };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer {
    pub list: Vec<char>,
    max_cursor_value: usize,
    cursor: usize,
}

impl utils::TomlIterator<char, Tokenizer> for Tokenizer {
    // next
    fn next(&mut self) -> Option<char> {
        if self.cursor > self.max_cursor_value {
            return None;
        } else {
            self.cursor += 1;
            return Some(self.list[self.cursor - 1].clone());
        }
    }
    // peek
    fn peek(&self) -> Option<char> {
        if self.cursor > self.max_cursor_value {
            return None;
        } else {
            return Some(self.list[self.cursor].clone());
        }
    }
    // peek_after
    fn peek_after(&self, n: usize) -> Option<char> {
        if self.cursor + n > self.max_cursor_value {
            return None;
        } else {
            return Some(self.list[self.cursor + n].clone());
        }
    }
    // peek_before
    fn peek_before(&self, n: usize) -> Option<char> {
        if (self.cursor as isize) - (n as isize) < 0 {
            return None;
        } else {
            return Some(self.list[self.cursor - n - 1].clone());
        }
    }
    // index
    fn index(&self, element: char) -> Option<usize> {
        let mut parser = Tokenizer::new(&self.list);
        while let Some(value) = parser.next() {
            if value == element {
                return Some(parser.cursor - 1);
            }
        }
        return None;
    }
}

impl Tokenizer {
    pub fn new(list: &Vec<char>) -> Tokenizer {
        let max_cursor_value = list.len() - 1;
        return Tokenizer {
            list: list.to_vec(),
            cursor: 0,
            max_cursor_value,
        };
    }
    pub fn tokenize_quote_string(&mut self) -> Token {
        let mut value = Vec::<char>::new();
        while let Some(character) = self.next() {
            if character == '"' {
                break;
            }
            value.push(character);
        }
        return Token::new(TokenKind::QuoteString, String::from_iter(value));
    }
    pub fn tokenize_nonquote_string(&mut self) -> Result<Token, String> {
        let mut value = Vec::<char>::new();
        while let Some(character) = self.peek() {
            if character.is_alphanumeric() || ['-', '_', ':'].contains(&character) {
                value.push(character);
                self.next();
            } else if [
                '!', '@', '$', '%', '^', '&', '*', '(', ')', '{', '}', ';', '<', '>', '|',
            ]
            .contains(&character)
            {
                return Err(format!("Unexpected character: {:#?}", character));
            } else {
                break;
            }
        }
        return Ok(Token::new(
            TokenKind::NonQuoteString,
            String::from_iter(value.clone()),
        ));
    }
}
