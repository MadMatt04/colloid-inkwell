use std::fmt::{Display, Formatter};
use std::fmt::Result as Result;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum TokenType {
    EndOfLine,
    EndOfFile,
    LeftParenthesis,
    RightParenthesis,
    Identifier
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "'{}'[{:?}]@{}:{}", self.lexeme, self.token_type, self.line, self.column)
    }
}