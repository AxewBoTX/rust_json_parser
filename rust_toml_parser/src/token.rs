use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        return Token { kind, value };
    }
    pub fn tokenize_quote_string(input: &mut Peekable<Chars<'_>>) -> Token {
        let mut value = Vec::<char>::new();
        while let Some(character) = input.next() {
            if character == '"' {
                break;
            }
            value.push(character);
        }
        return Token::new(TokenKind::QuoteString, String::from_iter(value));
    }
    pub fn tokenize_nonquote_string(input: &mut Peekable<Chars<'_>>) -> Result<Token, String> {
        let mut value = Vec::<char>::new();
        while let Some(character) = input.peek() {
            if character.is_alphanumeric() || ['-', '_', ':'].contains(character) {
                value.push(*character);
                input.next();
            } else if [
                '!', '@', '$', '%', '^', '&', '*', '(', ')', '{', '}', ';', '<', '>', '|',
            ]
            .contains(character)
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
