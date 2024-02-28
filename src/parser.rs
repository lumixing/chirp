use plex::parser;

use crate::lexer::{
    Span,
    Token::{self, *},
};

#[derive(Debug)]
pub enum Stmt_ {
    DeclareSprite(String, Vec<u8>),
    DeclareLabel(String),
    Nop,
    Clear,
    Return,
    JumpInteger(u16),
    JumpLabel(String),
    CallInteger(u16),
    CallLabel(String),
    SkipEqualsInteger(u8, u8),
    SkipNotEqualsInteger(u8, u8),
    SkipEqualsRegister(u8, u8),
    MoveRegisterInteger(u8, u8),
    AddRegisterInteger(u8, u8),
    MoveRegisterRegister(u8, u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    AddRegisterRegister(u8, u8),
    Subtract(u8, u8),
    ShiftRight(u8),
    SubtractReverse(u8, u8),
    ShiftLeft(u8),
    SkipNotEqualsRegister(u8, u8),
    MoveIRegisterInteger(u16),
    MoveIRegisterSprite(String),
    JumpRegister(u16),
    Random(u8, u8),
    Draw(u8, u8, u8),
    SkipKeyPressed(u8),
    SkipKeyNotPressed(u8),
    MoveRegisterDelay(u8),
    WaitKeyPress(u8),
    MoveDelayRegister(u8),
    MoveSoundRegister(u8),
    AddIRegisterRegister(u8),
    Sprite(u8),
    Bcd(u8),
    Save(u8),
    Load(u8),
}

#[derive(Debug)]
pub struct Stmt {
    pub span: Span,
    pub node: Stmt_,
}

pub struct Program {
    pub statements: Vec<Stmt>,
}

// #[allow(clippy::filter_map_next)]
parser! {
    fn parse_(Token, Span);

    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi
        }
    }

    program: Program {
        statements[s] => Program {
            statements: s
        }
    }

    statements: Vec<Stmt> {
        => vec![],
        statements[mut s] statement[st] Newline => {
            s.push(st);
            s
        },
        statements[s] Newline => s
    }

    hex: Vec<u8> {
        => vec![],
        hex[mut d] Int8(int) => {
            d.push(int);
            d
        }
    }

    statement: Stmt {
        Dollar Ident(id) hex[data] => Stmt {
            span: span!(),
            node: Stmt_::DeclareSprite(id, data)
        },
        Ident(id) Colon => Stmt {
            span: span!(),
            node: Stmt_::DeclareLabel(id),
        },
        Nop => Stmt {
            span: span!(),
            node: Stmt_::Nop,
        },
        Cls => Stmt {
            span: span!(),
            node: Stmt_::Clear,
        },
        Ret => Stmt {
            span: span!(),
            node: Stmt_::Return,
        },
        Jmp Int8(int) => Stmt {
            span: span!(),
            node: Stmt_::JumpInteger(int as u16),
        },
        Jmp Int16(int) => Stmt {
            span: span!(),
            node: Stmt_::JumpInteger(int),
        },
        Jmp Ident(id) => Stmt {
            span: span!(),
            node: Stmt_::JumpLabel(id),
        },
        Call Int8(nnn) => Stmt {
            span: span!(),
            node: Stmt_::CallInteger(nnn as u16),
        },
        Call Int16(nnn) => Stmt {
            span: span!(),
            node: Stmt_::CallInteger(nnn),
        },
        Call Ident(id) => Stmt {
            span: span!(),
            node: Stmt_::CallLabel(id),
        },
        Se Register(x) Comma Int8(nn) => Stmt {
            span: span!(),
            node: Stmt_::SkipEqualsInteger(x, nn),
        },
        Sne Register(x) Comma Int8(nn) => Stmt {
            span: span!(),
            node: Stmt_::SkipNotEqualsInteger(x, nn),
        },
        Se Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::SkipEqualsRegister(x, y),
        },
        Mov Register(x) Comma Int8(nn) => Stmt {
            span: span!(),
            node: Stmt_::MoveRegisterInteger(x, nn),
        },
        Add Register(x) Comma Int8(nn) => Stmt {
            span: span!(),
            node: Stmt_::AddRegisterInteger(x, nn),
        },
        Mov Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::MoveRegisterRegister(x, y),
        },
        Or Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::Or(x, y),
        },
        And Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::And(x, y),
        },
        Xor Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::Xor(x, y),
        },
        Add Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::AddRegisterRegister(x, y),
        },
        Sub Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::Subtract(x, y),
        },
        Shr Register(x) => Stmt {
            span: span!(),
            node: Stmt_::ShiftRight(x),
        },
        Subn Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::SubtractReverse(x, y),
        },
        Shl Register(x) => Stmt {
            span: span!(),
            node: Stmt_::ShiftLeft(x),
        },
        Sne Register(x) Comma Register(y) => Stmt {
            span: span!(),
            node: Stmt_::SkipNotEqualsRegister(x, y),
        },
        Mov IRegister Comma Int8(nnn) => Stmt {
            span: span!(),
            node: Stmt_::MoveIRegisterInteger(nnn as u16)
        },
        Mov IRegister Comma Int16(nnn) => Stmt {
            span: span!(),
            node: Stmt_::MoveIRegisterInteger(nnn)
        },
        Mov IRegister Comma Ident(id) => Stmt {
            span: span!(),
            node: Stmt_::MoveIRegisterSprite(id)
        },
        Jmpr Int8(nnn) => Stmt {
            span: span!(),
            node: Stmt_::JumpRegister(nnn as u16),
        },
        Jmpr Int16(nnn) => Stmt {
            span: span!(),
            node: Stmt_::JumpRegister(nnn),
        },
        Rnd Register(x) Comma Int8(nn) => Stmt {
            span: span!(),
            node: Stmt_::Random(x, nn),
        },
        Drw Register(x) Comma Register(y) Comma Int8(n) => Stmt {
            span: span!(),
            node: Stmt_::Draw(x, y, n),
        },
        Skp Register(x) => Stmt {
            span: span!(),
            node: Stmt_::SkipKeyPressed(x),
        },
        Sknp Register(x) => Stmt {
            span: span!(),
            node: Stmt_::SkipKeyNotPressed(x),
        },
        Mov Register(x) Comma DelayTimer => Stmt {
            span: span!(),
            node: Stmt_::MoveRegisterDelay(x),
        },
        Wait Register(x) => Stmt {
            span: span!(),
            node: Stmt_::WaitKeyPress(x),
        },
        Mov DelayTimer Comma Register(x) => Stmt {
            span: span!(),
            node: Stmt_::MoveDelayRegister(x),
        },
        Mov SoundTimer Comma Register(x) => Stmt {
            span: span!(),
            node: Stmt_::MoveSoundRegister(x),
        },
        Add IRegister Comma Register(x) => Stmt {
            span: span!(),
            node: Stmt_::AddIRegisterRegister(x),
        },
        Spr Register(x) => Stmt {
            span: span!(),
            node: Stmt_::Sprite(x),
        },
        Bcd Register(x) => Stmt {
            span: span!(),
            node: Stmt_::Bcd(x),
        },
        Save Register(x) => Stmt {
            span: span!(),
            node: Stmt_::Save(x),
        },
        Load Register(x) => Stmt {
            span: span!(),
            node: Stmt_::Load(x),
        },
    }

    nop: () {
        Newline => {}
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
