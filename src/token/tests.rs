use super::parse::parse;
use super::*;

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
                msg: "redundant ','".to_string(),
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

#[test]
fn test_bin_int_parsing() {
    let src = b"0b0110";

    let stream = parse(src).unwrap();
    assert_eq!(
        stream,
        vec![Token::Int {
            pos: Position {
                start: 0,
                end: 5,
                line: 1,
                column: 1,
                src: src
            }
        }]
    );

    let src = b"0b012";
    if let Err(err) = parse(src) {
        assert_eq!(
            err,
            Error {
                msg: "invalid character '2' in binary integer literal".to_string(),
                toks: vec![Token::Int {
                    pos: Position {
                        start: 4,
                        end: 4,
                        line: 1,
                        column: 1,
                        src: src
                    }
                }]
            }
        )
    } else {
        panic!("expected error")
    }
}
