use super::token::{Error, Position, Token};

struct Context {
    line: usize,
    indent: usize,
    idx: usize,            // Current buffer index
    nl_idx: Option<usize>, // Previous newline index
}

impl Context {
    // Returns column number for current index.
    fn col(&self) -> usize {
        match self.nl_idx {
            None => self.idx + 1,
            Some(nl) => self.idx - nl,
        }
    }
}

pub fn parse(src: &[u8]) -> Result<Vec<Token>, Error> {
    let mut toks: Vec<Token> = vec![];
    let mut ctx = Context {
        line: 1,
        indent: 0,
        idx: 0,
        nl_idx: None,
    };

    loop {
        if ctx.idx == src.len() {
            break;
        }

        let res = match src[ctx.idx] {
            b',' => parse_comma(&mut ctx, src),
            _ => todo!(),
        };

        match res {
            Ok(tok) => toks.push(tok),
            Err(err) => todo!(),
        }
    }

    Ok(toks)
}

fn parse_comma(ctx: &mut Context, src: &[u8]) -> Result<Token, Error> {
    let comma = Token::Comma {
        pos: Position {
            start: ctx.idx,
            end: ctx.idx,
            line: ctx.line,
            column: ctx.col(),
        },
    };
    ctx.idx += 1;
    Ok(comma)
}
