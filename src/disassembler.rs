use std::collections::{HashSet};

use crate::instructions::{Cmd, SetRegisterPair};
use crate::state::{RegisterPair};

pub fn instruction(memory: &Vec<u8>, at: usize) -> Option<Cmd> {
    let w = memory.get(at).expect("bad address");
    match format!("{:#X}", w).as_str() {
        "0x0" => Some(Cmd::Nop),
        "0x1" => Some(Cmd::Lxi(SetRegisterPair::BC(memory[at + 2], memory[at + 1]))),
        "0x11" => Some(Cmd::Lxi(SetRegisterPair::DE(memory[at + 2], memory[at + 1]))),
        "0x21" => Some(Cmd::Lxi(SetRegisterPair::HL(memory[at + 2], memory[at + 1]))),
        "0x31" => Some(Cmd::Lxi(SetRegisterPair::SP(address_at(memory, at)))),
        "0xC3" => Some(Cmd::Jmp(address_at(memory, at))),
        "0xC5" => Some(Cmd::Push(RegisterPair::BC)),
        "0xD5" => Some(Cmd::Push(RegisterPair::DE)),
        "0xE5" => Some(Cmd::Push(RegisterPair::HL)),
        "0xF5" => Some(Cmd::Push(RegisterPair::AF)),
        _ => None
    }
}

fn address_at(memory: &Vec<u8>, offset: usize) -> u16 {
    (memory[offset + 2] as u16) << 8 | memory[offset + 1] as u16
}

pub fn disassemble(memory: &Vec<u8>) {
    let mut offset = 0;
    let mut known_ops = 0;
    let mut total_ops = 0;

    let mut missing_opcodes_set = HashSet::new();

    while offset < memory.len() {
        let op = instruction(memory, offset);
        let skip_by = match &op {
            Some(cmd) => {
                println!("{:#X} - {}", offset, cmd);
                cmd.size()
            },
            None => {
                println!("{:#X} - ?? - {:#X}", offset, memory[offset]);
                1
            }
        };

        total_ops += 1;
        if op.is_some() {
            known_ops += 1;
        } else {
            missing_opcodes_set.insert(format!("{:#X}", memory[offset]));
        }

        offset += skip_by;
    }

    println!();
    println!("understood opcodes: {:?}% {}/{}", ((known_ops as f64 / total_ops as f64) * 100_f64).round(), known_ops , total_ops);

    let mut missing_opcodes_vec = missing_opcodes_set.iter().collect::<Vec<&String>>();
    missing_opcodes_vec.sort();

    println!("missing opcodes ({}): {:?}", missing_opcodes_set.len(), missing_opcodes_vec);
}