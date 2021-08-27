use std::collections::{HashSet};

use crate::instructions::{Cmd, SetRegisterPair};
use crate::state::{RegisterPair, Memory};

pub fn instruction(memory: Memory, at: u16) -> Option<Cmd> {
    let w = memory.get(at as usize).expect("bad address");
    match format!("{:#X}", w).as_str() {
        "0x0" => Some(Cmd::Nop),
        "0x1" => Some(Cmd::Lxi(SetRegisterPair::B(memory[(at + 2) as usize], memory[(at + 1) as usize]))),
        "0x11" => Some(Cmd::Lxi(SetRegisterPair::D(memory[(at + 2) as usize], memory[(at + 1) as usize]))),
        "0x21" => Some(Cmd::Lxi(SetRegisterPair::H(memory[(at + 2) as usize], memory[(at + 1) as usize]))),
        "0x31" => Some(Cmd::Lxi(SetRegisterPair::PSW(address_at(memory, at)))),
        "0xC3" => Some(Cmd::Jmp(address_at(memory, at))),
        "0xC5" => Some(Cmd::Push(RegisterPair::B)),
        "0xD5" => Some(Cmd::Push(RegisterPair::D)),
        "0xE5" => Some(Cmd::Push(RegisterPair::H)),
        "0xF5" => Some(Cmd::Push(RegisterPair::PSW)),
        _ => None
    }
}

fn address_at(memory: Memory, offset: u16) -> u16 {
    (memory[(offset + 2) as usize] as u16) << 8 | memory[(offset + 1) as usize] as u16
}

pub enum Opcode {
    Known(Cmd),
    Unknown(String)
}

pub fn disassemble(memory: Memory, max_address: u16) -> Vec<Opcode> {
    let mut offset = 0;
    let mut known_ops = 0;
    let mut total_ops = 0;

    let mut res = Vec::new();
    let mut missing_opcodes_set = HashSet::new();

    while offset <= max_address {
        let op = instruction(memory, offset);
        match &op {
            Some(cmd) => {
                println!("{:#X} - {}", offset, cmd);
                res.push(Opcode::Known(cmd.clone()));
            }
            None => {
                let opcode = format!("{:#X}", memory[offset as usize]);
                println!("{:#X} - ?? - {}", offset, opcode);
                res.push(Opcode::Unknown(opcode));
            }
        }

        let skip_by = match &op {
            Some(cmd) => cmd.size(),
            None => 1
        };

        total_ops += 1;
        if op.is_some() {
            known_ops += 1;
        } else {
            missing_opcodes_set.insert(format!("{:#X}", memory[offset as usize]));
        }

        offset += skip_by;
    }

    println!();
    println!("understood opcodes: {:?}% {}/{}", ((known_ops as f64 / total_ops as f64) * 100_f64).round(), known_ops , total_ops);

    let mut missing_opcodes_vec = missing_opcodes_set.iter().collect::<Vec<&String>>();
    missing_opcodes_vec.sort();

    println!("missing opcodes ({}): {:?}", missing_opcodes_set.len(), missing_opcodes_vec);

    res
}