use crate::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt::{Display, Formatter};

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
    UnexpectedInput(char)
}

pub type TokenResult = std::result::Result<Token, LexerError>;
pub type Tokens = Vec<TokenResult>;

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedInput(input) => write!(f, "Unexpected input: {}", input)
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
            },
            '(' => Ok(self.make_token(TokenType::LeftParenthesis)),
            ')' => Ok(self.make_token(TokenType::RightParenthesis)),
            _ => Err(LexerError::UnexpectedInput(c))
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme = self.source[self.start..self.current].to_owned();

        Token {
            token_type,
            lexeme,
            line: self.line,
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
                return;
            }
        }
    }
}

impl From<String> for Lexer {
    #[inline]
    fn from(source: String) -> Self {
        Lexer::new(source)
    }
}