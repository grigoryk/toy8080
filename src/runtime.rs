use crate::instructions::{Cmd, lxi};
use crate::state::{State};
use crate::disassembler::instruction;

pub fn execute(state: &mut State) {
    println!();
    println!("running:");
    let mut offset = 0;
    let op = instruction(&state.memory, offset);
    match op {
        Some(cmd) => run_op(offset, &cmd, state),
        None => panic!("unknown opcode: {:#X}", state.memory[offset])
    }
}

fn run_op(offset: usize, cmd: &Cmd, state: &mut State) {
    println!("{:#X} - {}", offset, cmd);
    match cmd {
        Cmd::Nop => {},
        Cmd::Lxi(srp) => lxi(state, srp),
        _ => panic!("unknown cmd {:?}", cmd)
    }
}