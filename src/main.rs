use std::env;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

mod state;
use state::{State, Registers, Flags};

mod instructions;
mod disassembler;
mod runtime;

fn main() {
    let mut state = State {
        registers: Registers {
            a: 0, b: 0, c: 0, d: 0, e: 0, l: 0, h: 0
        },
        flags: Flags {
            s: false, z: false, p: false, c: false, ac: false
        },
        sp: 0,
        pc: 0,
        memory: [0; u16::MAX as usize]
    };

    let args: Vec<String> = env::args().collect();
    let hfile = &args[1];
    println!("reading file: {:?}", hfile);
    println!("");
    let mut file = File::open(hfile).expect("Couldn't open the file");
    let fsize = match file.read(&mut state.memory[..]) {
        Ok(size) => size,
        Err(_) => panic!("Couldn't read file")
    };
    let opcodes = disassembler::disassemble(state.memory, u16::try_from(fsize).expect("file too large"));

    runtime::execute(&mut state, opcodes);
}