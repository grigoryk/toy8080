use std::env;
use std::fs;

mod state;
use state::{State, Registers, Flags};

mod instructions;

mod disassembler;
use disassembler::disassemble;

mod runtime;
use runtime::execute;

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
        memory: Vec::new()
    };

    let args: Vec<String> = env::args().collect();
    let hfile = &args[1];
    println!("reading file: {:?}", hfile);
    println!("");

    state.memory = fs::read(hfile).expect("Couldn't read the file");
    disassemble(&state.memory);

    // execute(&mut state);
}