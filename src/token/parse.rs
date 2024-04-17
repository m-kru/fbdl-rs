use super::*;

const VALID_AFTER_NUMBER: &'static [u8] = &[
    b' ', b'\t', b'\n', b'(', b')', b']', b'-', b'+', b'*', b'/', b'%', b'=', b'<', b'>', b';',
    b':', b',', b'|', b'&',
];

pub struct Context<'a> {
    pub src: &'a [u8],
    pub line: usize,
    pub indent: usize,
    pub idx: usize,            // Current buffer index
    pub nl_idx: Option<usize>, // Previous newline index
    pub toks: Vec<Token<'a>>,  // Token stream
}

impl<'a> Context<'a> {
    // Returns column number for current index.
    pub fn col(&self) -> usize {
        match self.nl_idx {
            None => self.idx + 1,
            Some(nl) => self.idx - nl,
        }
    }

    // Returns true if parsing is finished.
    fn end(&self) -> bool {
        self.idx == self.src.len()
    }

    // Returns byte from source with index equal idx.
    // If idx >= src.len(), then 0 is returned.
    fn byte(&self) -> u8 {
        if self.idx >= self.src.len() {
            0
        } else {
            self.src[self.idx]
        }
    }

    // Returns byte from source with index equal idx + 1.
    // If (idx + 1) >= src.len(), then 0 is returned.
    fn next_byte(&self) -> u8 {
        if self.idx + 1 >= self.src.len() {
            0
        } else {
            self.src[self.idx + 1]
        }
    }

    // Creates position from the current context state.
    fn pos(&self) -> Position<'a> {
        Position {
            start: self.idx,
            end: self.idx,
            line: self.line,
            column: self.col(),
            src: self.src,
        }
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
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                parse_number(&mut ctx)
            }
            b',' => parse_comma(&mut ctx),
            _ => todo!(),
        };

        let tok = res?;
        ctx.toks.push(tok);
    }

    Ok(ctx.toks)
}

fn parse_number<'a>(ctx: &mut Context<'a>) -> Result<Token<'a>, Error<'a>> {
    match (ctx.byte(), ctx.next_byte()) {
        (b'0', b'b') | (b'0', b'B') => parse_bin_int(ctx),
        _ => todo!(),
    }
}

fn parse_bin_int<'a>(ctx: &mut Context<'a>) -> Result<Token<'a>, Error<'a>> {
    let mut pos = ctx.pos();

    // Skip 0b
    ctx.idx += 2;

    loop {
        if ctx.end() {
            break;
        }

        match ctx.byte() {
            b'0' | b'1' => ctx.idx += 1,
            byte if VALID_AFTER_NUMBER.contains(&byte) => break,
            byte => {
                pos.start = ctx.idx;
                pos.end = ctx.idx;
                return Err(Error {
                    msg: format!(
                        "invalid character '{}' in binary integer literal",
                        byte as char
                    ),
                    toks: vec![Token::Int { pos }],
                });
            }
        }
    }

    pos.end = ctx.idx - 1;
    Ok(Token::Int { pos })
}

fn parse_comma<'a>(ctx: &mut Context<'a>) -> Result<Token<'a>, Error<'a>> {
    let comma = Token::Comma { pos: ctx.pos() };

    if let Some(Token::Comma { .. }) = ctx.toks.last() {
        Err(Error {
            msg: "redundant ','".to_string(),
            toks: vec![comma],
        })
    } else {
        ctx.idx += 1;
        Ok(comma)
    }
}
