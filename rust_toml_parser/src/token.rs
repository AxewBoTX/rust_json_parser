#[derive(Debug)]
pub enum TokenKind {
    CurleyBracketOpen,
    CurleyBracketClose,
    BracketOpen,
    BracketClose,
    EqualTo,
    Comma,
    String,
    Number,
    True,
    False,
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
}
