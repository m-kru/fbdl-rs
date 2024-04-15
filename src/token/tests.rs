use super::parse::parse;
use super::token::{Position, Token};

#[test]
fn test_comma_parsing() {
    let src = b",";

    let stream = parse(src).unwrap();
    assert_eq!(
        stream,
        vec![Token::Comma {
            pos: Position {
                start: 0,
                end: 0,
                line: 1,
                column: 1
            }
        }]
    )
}
