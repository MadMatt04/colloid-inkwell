use std::fmt::Result;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum TokenType {
    Error,
    EndOfLine,
    EndOfFile,
    LeftParenthesis,
    RightParenthesis,
    Identifier,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

impl Token {
    #[inline]
    pub fn is_error(&self) -> bool {
        self.token_type == TokenType::Error
    }

    #[inline]
    pub fn is_regular(&self) -> bool {
        !self.is_error()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "'{}'[{:?}]@{}:{}",
            self.lexeme, self.token_type, self.line, self.column
        )
    }
}
