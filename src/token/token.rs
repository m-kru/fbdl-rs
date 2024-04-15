#[derive(Debug, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Error {
    msg: String,
    tokens: Vec<Token>,
}

macro_rules! tokens {
    ( $( $name:ident ),* ) => {
        #[derive(Debug, PartialEq)]
        pub enum Token {
            $( $name{pos: Position}, )*
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
    Celay,
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
