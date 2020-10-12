use std::env;
use std::fs;

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

    let file = fs::read(hfile).expect("Couldn't read the file");
    let mut current_address: u16 = 0;
    for byte in file {
        state.memory[current_address as usize] = byte;
        current_address += 1;
        if current_address > u16::MAX {
            panic!("memory overflow while reading file");
        }
    }
    let opcodes = disassembler::disassemble(state.memory, current_address);

    runtime::execute(&mut state, opcodes);
}