use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    pub msg: String,
    pub toks: Vec<Token<'a>>,
}
