use crate::instructions::{Cmd, lxi};
use crate::state::{State};
use crate::disassembler::{Opcode};

pub fn execute(state: &mut State, opcodes: Vec<Opcode>) {
    println!();
    println!("running:");

    let mut current = opcodes.get(state.pc as usize);
    loop {
        state.pc = match current {
            Some(Opcode::Known(cmd)) => run_cmd(state.pc, cmd, state),
            Some(Opcode::Unknown(opcode)) => panic!("Unknown opcode: {}", opcode),
            None => {
                println!("{:#X} outside of opcodes", state.pc);
                break
            }
        };
        current = opcodes.get(state.pc as usize);
    }
    println!("finished");

    // state.sp

    // loop {
    // opcodes.iter();
    // }

    // match op {
    //     Some(cmd) => run_op(offset, &cmd, state),
    //     None => panic!("unknown opcode: {:#X}", state.memory[offset])
    // }
}

fn run_cmd(pc: u16, cmd: &Cmd, state: &mut State) -> u16 {
    println!("{:#X} - {}", pc, cmd);
    match cmd {
        Cmd::Nop => pc + 1,
        Cmd::Lxi(srp) => {
            lxi(state, srp);
            pc + 1
        },
        Cmd::Jmp(address) => *address,
        _ => panic!("unknown cmd {:?}", cmd)
    }
}