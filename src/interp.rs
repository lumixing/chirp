use crate::{
    parser::{Program, Stmt, Stmt_::*},
    Diagnostic,
};
use std::collections::HashMap;

struct Props<'a> {
    pub pc: u16,
    pub ins: Vec<u8>,
    pub labels: HashMap<&'a str, u16>,
    pub sprites: HashMap<&'a str, u16>,
    pub sprite_data: Vec<u8>,
    pub line: usize,
    pub skips: usize,
    pub code_size: usize,
}

const PROGRAM_START: usize = 0x200;

pub fn interp<'a>(program: &'a Program, diagnostic: &Diagnostic) -> Vec<u8> {
    let mut props = Props {
        pc: 0,
        ins: vec![],
        labels: HashMap::new(),
        sprites: HashMap::new(),
        sprite_data: vec![],
        line: 0,
        skips: 0,
        code_size: 0,
    };

    // first phase: labels and code size
    for expr in &program.statements {
        props.pc += 2;
        props.line += 1;
        interp_label(&mut props, diagnostic, expr);
    }

    props.code_size = (program.statements.len() - props.skips) * 2;
    props.pc = 0;
    props.line = 0;

    for expr in &program.statements {
        props.pc += 2;
        props.line += 1;
        interp_stmt(&mut props, diagnostic, expr);
    }

    props.ins.extend(props.sprite_data);
    props.ins
}

fn interp_label<'a>(props: &mut Props<'a>, diagnostic: &Diagnostic, stmt: &'a Stmt) {
    match stmt.node {
        DeclareLabel(ref id) => {
            if props.labels.contains_key(id.as_str()) {
                diagnostic.warn(stmt.span, format!("label {:?} is already declared", id));
            }

            props.labels.insert(id, PROGRAM_START as u16 + props.pc - 2);
            props.pc -= 2;
            props.skips += 1;
        }
        DeclareSprite(_, _) => {
            props.pc -= 2;
            props.skips += 1;
        }
        _ => {}
    }
}

fn interp_stmt<'a>(props: &mut Props<'a>, diagnostic: &Diagnostic, stmt: &'a Stmt) {
    match stmt.node {
        DeclareLabel(_) => {}
        DeclareSprite(ref id, ref data) => {
            if props.sprites.contains_key(id.as_str()) {
                diagnostic.warn(stmt.span, format!("sprite {:?} is already declared", id));
            }

            let sprite_location = PROGRAM_START + props.code_size + props.sprite_data.len();
            props.sprites.insert(id, sprite_location as u16);
            props.sprite_data.extend(data);
            props.pc -= 2;
        }
        Nop => {
            props.ins.extend(vec![0x00, 0x00]);
        }
        Clear => {
            props.ins.extend(vec![0x00, 0xE0]);
        }
        Return => {
            props.ins.extend(vec![0x00, 0xEE]);
        }
        JumpInteger(ref nnn) => {
            let high_byte = 0x10 + ((nnn & 0xF00) >> 8);
            let low_byte = nnn & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        JumpLabel(ref id) => {
            if !props.labels.contains_key(id.as_str()) {
                diagnostic.error(stmt.span, format!("label {:?} is not declared", id));
            }

            let pc = props.labels.get(id.as_str()).unwrap();
            let high_byte = 0x10 + ((pc & 0xF00) >> 8);
            let low_byte = pc & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        CallInteger(ref nnn) => {
            let high_byte = 0x20 + ((nnn & 0xF00) >> 8);
            let low_byte = nnn & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        CallLabel(ref id) => {
            if !props.labels.contains_key(id.as_str()) {
                diagnostic.error(stmt.span, format!("label {:?} is not declared", id));
            }

            let pc = props.labels.get(id.as_str()).unwrap();
            let high_byte = 0x20 + ((pc & 0xF00) >> 8);
            let low_byte = pc & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipEqualsInteger(ref x, ref nn) => {
            let high_byte = 0x30 + x;
            let low_byte = *nn;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipNotEqualsInteger(ref x, ref nn) => {
            let high_byte = 0x40 + x;
            let low_byte = *nn;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipEqualsRegister(ref x, ref y) => {
            let high_byte = 0x50 + x;
            let low_byte = y << 4;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveRegisterInteger(ref x, ref nn) => {
            let high_byte = 0x60 + x;
            let low_byte = *nn;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        AddRegisterInteger(ref x, ref nn) => {
            let high_byte = 0x70 + x;
            let low_byte = *nn;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveRegisterRegister(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = y << 4;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Or(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 1;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        And(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 2;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Xor(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 3;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        AddRegisterRegister(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 4;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Subtract(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 5;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        ShiftRight(ref x) => {
            let high_byte = 0x80 + x;
            let low_byte = 0x06;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SubtractReverse(ref x, ref y) => {
            let high_byte = 0x80 + x;
            let low_byte = (y << 4) + 7;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        ShiftLeft(ref x) => {
            let high_byte = 0x80 + x;
            let low_byte = 0x0E;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipNotEqualsRegister(ref x, ref y) => {
            let high_byte = 0x90 + x;
            let low_byte = y << 4;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveIRegisterInteger(ref nnn) => {
            let high_byte = 0xA0 + ((nnn & 0xF00) >> 8);
            let low_byte = nnn & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveIRegisterSprite(ref id) => {
            if !props.sprites.contains_key(id.as_str()) {
                diagnostic.error(stmt.span, format!("sprite {:?} is not declared", id));
            }

            let pc = props.sprites.get(id.as_str()).unwrap();
            let high_byte = 0xA0 + ((pc & 0xF00) >> 8);
            let low_byte = pc & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        JumpRegister(ref nnn) => {
            let high_byte = 0xB0 + ((nnn & 0xF00) >> 8);
            let low_byte = nnn & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Random(ref x, ref nn) => {
            let high_byte = 0xC0 + x;
            let low_byte = *nn;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Draw(ref x, ref y, ref n) => {
            let high_byte = 0xD0 + x;
            let low_byte = (y << 4) + n;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipKeyPressed(ref x) => {
            let high_byte = 0xE0 + x;
            let low_byte = 0x9E;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        SkipKeyNotPressed(ref x) => {
            let high_byte = 0xE0 + x;
            let low_byte = 0xA1;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveRegisterDelay(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x07;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        WaitKeyPress(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x0A;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveDelayRegister(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x15;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        MoveSoundRegister(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x18;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        AddIRegisterRegister(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x1E;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Sprite(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x29;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Bcd(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x33;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Save(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x55;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        Load(ref x) => {
            let high_byte = 0xF0 + x;
            let low_byte = 0x65;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
    }
}
