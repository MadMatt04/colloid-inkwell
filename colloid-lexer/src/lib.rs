mod tokens;
mod lexer;

pub use tokens::{TokenType, Token};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
