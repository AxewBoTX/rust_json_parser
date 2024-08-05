use crate::utils;

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
    pub list: utils::IteratorList<char>,
}

impl Tokenizer {
    pub fn new(list: &Vec<char>) -> Tokenizer {
        return Tokenizer {
            list: utils::IteratorList::new(list),
        };
    }
    pub fn tokenize_quote_string(&mut self) -> Token {
        let mut value = Vec::<char>::new();
        while let Some(character) = self.list.current() {
            if character == '"' {
                break;
            }
            value.push(character);
        }
        return Token::new(TokenKind::QuoteString, String::from_iter(value));
    }
    pub fn tokenize_nonquote_string(&mut self) -> Result<Token, String> {
        let mut value = Vec::<char>::new();
        while let Some(character) = self.list.peek() {
            if character.is_alphanumeric() || ['-', '_', ':'].contains(&character) {
                value.push(character);
                self.list.current();
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
