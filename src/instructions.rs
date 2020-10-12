use std::fmt;
use crate::state::{State, RegisterPair};

#[derive(Clone, Debug)]
pub enum SetRegisterPair {
    BC(u8, u8),
    DE(u8, u8),
    HL(u8, u8),
    SP(u16)
}

impl fmt::Display for SetRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetRegisterPair::BC(b1, b2) => write!(f, "B={:#X}, C={:#X}", b1, b2),
            SetRegisterPair::DE(b1, b2) => write!(f, "D={:#X}, E={:#X}", b1, b2),
            SetRegisterPair::HL(b1, b2) => write!(f, "H={:#X}, L={:#X}", b1, b2),
            SetRegisterPair::SP(a) => write!(f, "SP={:#X}", a),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cmd {
    Nop,

    // e.g. B <- byte 3, C <- byte 2
    Lxi(SetRegisterPair),
    Jmp(u16), // PC <= adr
    Push(RegisterPair),

    // StaxB, // (BC) <- A
    // StaxD, // (DE) <- A
    // Shld, // (adr) <-L; (adr+1)<-H

    // PushB, // (sp-2)<-C; (sp-1)<-B; sp <- sp - 2
    // PushD, // (sp-2)<-E; (sp-1)<-D; sp <- sp - 2
    // PushH, // (sp-2)<-L; (sp-1)<-H; sp <- sp - 2
    // PushPsw, // (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2

    // AddB, // A <- A + B
    // AddC, // A <- A + C

    // DadD, // HL = HL + DE
    // DcrD, // D <- D-1
    // DcrE, // E <- E-1
    // DcrH, // H <- H-1

    // InrE, // E <-E+1
    // InrD, // D <- D+1
    // InrH, // H <- H+1
    // InxD, // DE <- DE + 1
    // InxH, // HL <- HL + 1

    // MviA, // A <- byte 2
    // MviD, // D <- byte 2
    // MviE, // E <- byte 2
    // MviL, // L <- byte 2

    // Rst7, // CALL $38
    // Rz, // if Z, RET

    // SbbC, // A <- A - C - CY

    // Jz, // if Z, PC <- adr
    // Jc, // if CY, PC<-adr
    // Jnz, // if NZ, PC <- adr
    // Jm, // if M, PC <- adr

    // Cpo, // if PO, CALL adr

    // MovAD, // A <- D

    // MovCA, // C <- A
    // MovCB, // C <- B
    // MovCC, // C <- C
    // MovCD, // C <- D
    // MovCH, // C <- H
    // MovCL, // C <- H
    // MovCE, // C <- E
    // MovCM, // C <- M

    // MovHA, // H <- A
    // MovHM, // H <- (HL)

    // Pchl, // PC.hi <- H; PC.lo <- L

    // Ral, // A = A << 1; bit 0 = prev CY; CY = prev bit 7
    // Rar, // A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0

    // SubM, // A <- A + (HL)

    // Xthl, // L <-> (SP); H <-> (SP+1)

    // LdaxD, // A <- (DE)
    // DcxD, // DE = DE-1

    Di, // special
    Unused, // ???
}

impl Cmd {
    pub fn size(&self) -> usize {
        match self {
            Cmd::Nop => 1,
            Cmd::Jmp(_) => 3,
            _ => 1
        }
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmd::Jmp(x) => write!(f, "Jmp({:#X})", x),
            Cmd::Lxi(x) => write!(f, "Lxi({})", x),
            _ => write!(f, "{:?}", self)
        }
    }
}

pub fn lxi(state: &mut State, srp: &SetRegisterPair) {
    match srp {
        SetRegisterPair::BC(b1, b2) => {
            state.registers.b = *b1;
            state.registers.c = *b2;
        },
        SetRegisterPair::DE(b1, b2) => {
            state.registers.d = *b1;
            state.registers.e = *b2;
        },
        SetRegisterPair::HL(b1, b2) => {
            state.registers.h = *b1;
            state.registers.l = *b2;
        },
        SetRegisterPair::SP(a) => {
            state.sp = *a
        }
    }
}
