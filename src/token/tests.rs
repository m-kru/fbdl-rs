use super::error::Error;
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
                column: 1,
                src: src
            }
        }]
    );

    let src = b",,";
    if let Err(err) = parse(src) {
        assert_eq!(
            err,
            Error {
                msg: "redundant ','",
                toks: vec![Token::Comma {
                    pos: Position {
                        start: 1,
                        end: 1,
                        line: 1,
                        column: 2,
                        src: src
                    }
                }]
            }
        )
    } else {
        panic!("expected error")
    }
}
