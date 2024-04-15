use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    pub msg: &'static str,
    pub toks: Vec<Token<'a>>,
}
