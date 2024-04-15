use super::error::*;
use super::token::*;

struct Context<'a> {
    src: &'a [u8],
    line: usize,
    indent: usize,
    idx: usize,            // Current buffer index
    nl_idx: Option<usize>, // Previous newline index
    toks: Vec<Token<'a>>,  // Token stream
}

impl Context<'_> {
    // Returns column number for current index.
    fn col(&self) -> usize {
        match self.nl_idx {
            None => self.idx + 1,
            Some(nl) => self.idx - nl,
        }
    }

    // Returns true if parsing is finished.
    fn end(&self) -> bool {
        self.idx == self.src.len()
    }
}

pub fn parse(src: &[u8]) -> Result<Vec<Token>, Error> {
    let mut ctx = Context {
        src: src,
        line: 1,
        indent: 0,
        idx: 0,
        nl_idx: None,
        toks: vec![],
    };

    loop {
        if ctx.end() {
            break;
        }

        let res = match src[ctx.idx] {
            b',' => parse_comma(&mut ctx),
            _ => todo!(),
        };

        let tok = res?;
        ctx.toks.push(tok);
    }

    Ok(ctx.toks)
}

fn parse_comma<'a>(ctx: &mut Context<'a>) -> Result<Token<'a>, Error<'a>> {
    let comma = Token::Comma {
        pos: Position {
            start: ctx.idx,
            end: ctx.idx,
            line: ctx.line,
            column: ctx.col(),
            src: ctx.src,
        },
    };

    if let Some(Token::Comma { .. }) = ctx.toks.last() {
        Err(Error {
            msg: "redundant ','",
            toks: vec![comma],
        })
    } else {
        ctx.idx += 1;
        Ok(comma)
    }
}
