mod parse;
mod tests;

#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    pub msg: String,
    pub toks: Vec<Token<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Position<'a> {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub src: &'a [u8],
}

macro_rules! tokens {
    ( $( $name:ident ),* ) => {
        #[derive(Debug, PartialEq)]
        pub enum Token<'a> {
            $( $name{pos: Position<'a>}, )*
        }
    };
}

tokens! {
    Comment,
    Indent,
    Dedent,
    Newline,
    Eof,
    Identifier,
    QualifiedIdentifier,
    Bool,
    Int,
    Real,
    String,
    BitString, // b"101", o"705", x"beef"
    Time,
    Negation, // !
    Assignment, // =
    Addition, // +
    Subtraction, // -
    Multiplication, // *
    Division, // /
    Modulo, // %
    Exponent, // **
    Equality, // ==
    NonEquality, // !=
    Less, // <
    LessEqual, // <=
    Greater, // >
    GreaterEqual, // >=
    And, // &&
    Or, // ||
    LeftShift, // <<
    RightShift, // >>
    BitAnd, // &
    BitOr, // |
    BitXor, // ^
    LeftParenthesis, // (
    RightParenthesis, // )
    LeftBracket, // [
    RightBracket, // ]
    Comma, // ,
    Semicolon, // ;
    // Keywords
    Const,
    Import,
    Type,
    // Functionalities
    Block,
    Bus,
    Config,
    Irq,
    Mask,
    Memory,
    Param,
    Proc,
    Return,
    Static,
    Status,
    Stream,
    // Properties
    Access,
    AddEnable,
    Atomic,
    ByteWriteEnable,
    Clear,
    Delay,
    EnableInitValue,
    EnableResetValue,
    Groups,
    InitBalue,
    InTrigger,
    Masters,
    OutTrigger,
    Range,
    ReadLatency,
    ReadBalue,
    Reset,
    ResetValue,
    Size,
    Width,
    Period
    // Currently unused
    // period, // .
    // colon, // :
    // left_brace, // {
    // right_brace, // {
}
