use crate::parser::{Program, Stmt, Stmt_::*};
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

pub fn interp<'a>(p: &'a Program) -> Vec<u8> {
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
    for expr in &p.statements {
        props.pc += 2;
        props.line += 1;
        interp_label(&mut props, expr);
    }

    props.code_size = (p.statements.len() - props.skips) * 2;
    props.pc = 0;
    props.line = 0;

    for expr in &p.statements {
        props.pc += 2;
        props.line += 1;
        interp_stmt(&mut props, expr);
    }

    props.ins.extend(props.sprite_data);
    props.ins
}

fn interp_label<'a>(props: &mut Props<'a>, stmt: &'a Stmt) {
    match stmt.node {
        DeclareLabel(ref id) => {
            if props.labels.contains_key(id.as_str()) {
                println!("WARN: label {:?} already exists, overriding", id);
            }

            props.labels.insert(id, props.pc);
            props.pc -= 2;
            props.skips += 1;
        }
        DeclareSprite(_, _) => {
            props.skips += 1;
        }
        _ => {}
    }
}

fn interp_stmt<'a>(props: &mut Props<'a>, stmt: &'a Stmt) {
    match stmt.node {
        DeclareLabel(_) => {}
        DeclareSprite(ref id, ref data) => {
            if props.sprites.contains_key(id.as_str()) {
                println!(
                    "WARN: sprite {:?} already exists, overriding at line {}",
                    id, props.line
                );
            }

            let sprite_location = PROGRAM_START + props.code_size + props.sprite_data.len();
            props.sprites.insert(id, sprite_location as u16);
            props.sprite_data.extend(data);
            props.pc -= 2;
        }
        Clear => {
            props.ins.extend(vec![0x00, 0xE0]);
        }
        MoveIRegisterSprite(ref id) => {
            if !props.sprites.contains_key(id.as_str()) {
                panic!(
                    "ERROR: could not find sprite {:?} at line {}",
                    id, props.line
                );
            }

            let pc = props.sprites.get(id.as_str()).unwrap();
            let high_byte = 0xA0 + ((pc & 0xF00) >> 8);
            let low_byte = pc & 0x0FF;
            props.ins.extend(vec![high_byte as u8, low_byte as u8]);
        }
        _ => unimplemented!(),
    }
}
