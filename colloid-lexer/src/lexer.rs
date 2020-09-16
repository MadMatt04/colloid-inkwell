use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedInput(Token)
}

pub type TokenResult = std::result::Result<Token, LexerError>;
pub type Tokens = Vec<TokenResult>;

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedInput(input) => write!(f, "Unexpected input: {}", input.lexeme)
        }
    }
}

impl Error for LexerError {}

impl Lexer {
    #[inline]
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn lex(&mut self) -> Tokens {
        let mut tokens = Tokens::new();

        loop {
            let token = self.scan_token();
            let mut end = false;
            if let Ok(t) = &token {
                if t.token_type == TokenType::EndOfFile {
                    end = true;
                }
            }

            tokens.push(token);

            if end {
                break;
            }
        }

        //tokens.push(self.make_token(TokenType::EndOfFile));
        tokens
    }

    fn scan_token(&mut self) -> TokenResult {
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.make_token(TokenType::EndOfFile));
        }

        self.skip_whitespace();

        let c = self.advance();

        match c {
            '\n' => {
                let nl = self.make_token(TokenType::EndOfLine);
                self.line += 1;
                self.column = 0;
                Ok(nl)
            }
            '(' => Ok(self.make_token(TokenType::LeftParenthesis)),
            ')' => Ok(self.make_token(TokenType::RightParenthesis)),
            _ => Err(LexerError::UnexpectedInput(self.make_token(TokenType::Error)))
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme = self.source[self.start..self.current].to_owned();

        Token {
            token_type,
            lexeme,
            line: self.line,
            //column: if token_type == TokenType::EndOfFile {0} else {self.column - 1},
            column: self.column,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }

        self.start = self.current;
    }
}

impl From<String> for Lexer {
    #[inline]
    fn from(source: String) -> Self {
        Lexer::new(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_some_parenthesis() {
        let mut lexer = Lexer::new(String::from("   (\t\n )\n"));
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].as_ref().unwrap().clone(), Token {
            token_type: TokenType::LeftParenthesis,
            lexeme: String::from("("),
            line: 1,
            column: 4,
        });
        assert_eq!(tokens[1].as_ref().unwrap().clone(), Token {
            token_type: TokenType::EndOfLine,
            lexeme: String::from("\n"),
            line: 1,
            column: 6,
        });
        assert_eq!(tokens[2].as_ref().unwrap().clone(), Token {
            token_type: TokenType::RightParenthesis,
            lexeme: String::from(")"),
            line: 2,
            column: 2,
        });
        assert_eq!(tokens[3].as_ref().unwrap().clone(), Token {
            token_type: TokenType::EndOfLine,
            lexeme: String::from("\n"),
            line: 2,
            column: 3,
        });
        assert_eq!(tokens[4].as_ref().unwrap().clone(), Token {
            token_type: TokenType::EndOfFile,
            lexeme: String::from(""),
            line: 3,
            column: 0,
        });
    }
}