#[derive(Clone, Debug)]
pub enum Register {
    A, B, C, D, E, H, L
}

#[derive(Clone, Debug)]
pub enum RegisterPair {
    B, D, H, PSW // PSW - Program Status Word, refers to A and flags
}

pub struct Flags {
    pub s: bool, // sign
    pub z: bool, // zero
    pub p: bool, // parity
    pub c: bool, // carry
    pub ac: bool // aux carry
}

pub struct Registers {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a: u8, // accumulator
}

pub type Memory = [u8; u16::MAX as usize];

pub struct State {
    pub registers: Registers,
    pub sp: u16, // stack pointer
    pub pc: u16, // program counter
    pub flags: Flags,
    pub memory: Memory
}
