use plex::lexer;

#[derive(Debug)]
pub enum Token {
    Whitespace,
    Newline,
    Comment,
    Comma,
    Colon,

    Nop,
    Cls,
    Ret,
    Jmp,
    Call,
    Se,
    Sne,
    Mov,
    Add,
    Or,
    And,
    Xor,
    Sub,
    Shr,
    Subn,
    Shl,
    Jmpr,
    Rnd,
    Drw,
    Skp,
    Sknp,
    Wait,
    Spr,
    Bcd,
    Save,
    Load,

    Register(u8),
    Int8(u8),
    Int16(u16),
    IRegister,

    Ident(String),
}

lexer! {
    fn next_token(tok: 'a) -> Token;

    r#"[ \r\t]+"# => Token::Whitespace,
    r#"\n"# => Token::Newline,
    r#"#[^\n]*"# => Token::Comment,
    r#","# => Token::Comma,
    r#":"# => Token::Colon,

    r#"nop"#  => Token::Nop,
    r#"cls"#  => Token::Cls,
    r#"ret"#  => Token::Ret,
    r#"jmp"#  => Token::Jmp,
    r#"call"# => Token::Call,
    r#"se"#   => Token::Se,
    r#"sne"#  => Token::Sne,
    r#"mov"#  => Token::Mov,
    r#"add"#  => Token::Add,
    r#"or"#   => Token::Or,
    r#"and"#  => Token::And,
    r#"xor"#  => Token::Xor,
    r#"sub"#  => Token::Sub,
    r#"shr"#  => Token::Shr,
    r#"subn"# => Token::Subn,
    r#"shl"#  => Token::Shl,
    r#"jmpr"# => Token::Jmpr,
    r#"rnd"#  => Token::Rnd,
    r#"drw"#  => Token::Drw,
    r#"skp"#  => Token::Skp,
    r#"sknp"# => Token::Sknp,
    r#"wait"# => Token::Wait,
    r#"spr"#  => Token::Spr,
    r#"bcd"#  => Token::Bcd,
    r#"save"# => Token::Save,
    r#"load"# => Token::Load,

    r#"v[0-9][0-5]?"# => {
        let idx: u8 = tok.trim_matches('v').parse()
            .expect("could not parse register idx");
        Token::Register(idx)
    },

    r#"[0-9]+"# => {
        let int: u16 = tok.parse()
            .expect("could not parse dec");

        match int {
            0..=255 => Token::Int8(int as u8),
            0..=u16::MAX => Token::Int16(int)
        }
    },

    r#"0x[0-9a-f]+"# => {
        let int = u16::from_str_radix(tok.trim_start_matches("0x"), 16)
            .expect("could not parse hex");

        match int {
            0..=255 => Token::Int8(int as u8),
            0..=u16::MAX => Token::Int16(int)
        }
    },

    r#"i"# => Token::IRegister,
    r#"[a-zA-Z_]+"# => Token::Ident(tok.to_string()),

    r#"."# => panic!("invalid character: {:?}", tok)
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
