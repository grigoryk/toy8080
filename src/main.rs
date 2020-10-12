use std::env;
use std::fs;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let hfile = &args[1];
    println!("disassembling file: {:?}", hfile);
    println!("");

    let optable = table();

    let contents = fs::read(hfile).expect("Couldn't read the file");

    let mut offset = 0;
    let length = contents.len();
    let mut known_ops = 0;
    let mut total_ops = 0;

    let mut missing_cmds = HashSet::new();

    while offset < length {
        let op = format!("{:#X}", contents[offset]);
        let known_op = optable.get(op.as_str());
        let skip_by = match known_op {
            Some(x) => x.size(),
            None => 1 // assume size=1 when we don't know the opCmd yet
        };
        total_ops += 1;
        if known_op.is_some() {
            known_ops += 1;
        }
        // note that addresses are stored backwards, little endian.
        match known_op {
            Some(op @ OpCmd { name: _, argument_type: ArgType::None}) => {
                println!("{:#X} {:?}", total_ops, op.name)
            },
            Some(op @ OpCmd { name: _, argument_type: ArgType::Address}) => {
                println!("{:#X} {:?} - addr: {:#X}{:X}", total_ops, op.name, contents[offset + 2], contents[offset + 1])
            },
            Some(op @ OpCmd { name: _, argument_type: ArgType::ArgOne}) => {
                println!("{:#X} {:?} - arg1: {:#X}", total_ops, op.name, contents[offset + 1])
            },
            Some(op @ OpCmd { name: _, argument_type: ArgType::ArgTwo}) => {
                println!("{:#X} {:?} - arg1: {:#X}, arg2: {:#X}", total_ops, op.name, contents[offset + 2], contents[offset + 1])
            },
            None => {
                println!("{:#X} ??", total_ops);
                missing_cmds.insert(op);
            }
        }
        offset += skip_by;
    }

    println!();
    println!("understood commands: {:?}% {}/{}", ((known_ops as f64 / total_ops as f64) * 100_f64).round(), known_ops , total_ops);

    let mut missing_cmds_vec = missing_cmds.iter().collect::<Vec<&String>>();
    missing_cmds_vec.sort();

    println!("missing opCmds ({}): {:?}", missing_cmds.len(), missing_cmds_vec);
}

#[derive(Clone)]
struct OpCmd {
    name: Cmd,
    argument_type: ArgType
}

impl OpCmd {
    fn new(name: Cmd, argument_type: ArgType) -> OpCmd {
        OpCmd { name, argument_type }
    }

    fn size(&self) -> usize {
        match self.argument_type {
            ArgType::None => 1,
            ArgType::ArgOne => 2,
            ArgType::ArgTwo | ArgType::Address => 3,
        }
    }
}

#[derive(Clone, Debug)]
enum ArgType {
    None,
    Address,
    ArgOne,
    ArgTwo,
}

#[derive(Clone, Debug)]
enum Cmd {
    Nop,

    LxiB, // B <- byte 3, C <- byte 2
    LxiD, // D <- byte 3, E <- byte 2
    LxiH, // H <- byte 3, L <- byte 2
    LxiSp, // SP.hi <- byte 3, SP.lo <- byte 2

    StaxB, // (BC) <- A
    StaxD, // (DE) <- A
    Shld, // (adr) <-L; (adr+1)<-H
    Jmp, // PC <= adr

    PushB, // (sp-2)<-C; (sp-1)<-B; sp <- sp - 2
    PushD, // (sp-2)<-E; (sp-1)<-D; sp <- sp - 2
    PushH, // (sp-2)<-L; (sp-1)<-H; sp <- sp - 2
    PushPsw, // (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2

    AddB, // A <- A + B
    AddC, // A <- A + C

    DadD, // HL = HL + DE
    DcrD, // D <- D-1
    DcrE, // E <- E-1
    DcrH, // H <- H-1

    InrE, // E <-E+1
    InrD, // D <- D+1
    InrH, // H <- H+1
    InxD, // DE <- DE + 1
    InxH, // HL <- HL + 1

    MviA, // A <- byte 2
    MviD, // D <- byte 2
    MviE, // E <- byte 2
    MviL, // L <- byte 2

    Rst7, // CALL $38
    Rz, // if Z, RET

    SbbC, // A <- A - C - CY

    Jz, // if Z, PC <- adr
    Jc, // if CY, PC<-adr
    Jnz, // if NZ, PC <- adr
    Jm, // if M, PC <- adr

    Cpo, // if PO, CALL adr

    MovAD, // A <- D

    MovCA, // C <- A
    MovCB, // C <- B
    MovCC, // C <- C
    MovCD, // C <- D
    MovCH, // C <- H
    MovCL, // C <- H
    MovCE, // C <- E
    MovCM, // C <- M

    MovHA, // H <- A
    MovHM, // H <- (HL)

    Pchl, // PC.hi <- H; PC.lo <- L

    Ral, // A = A << 1; bit 0 = prev CY; CY = prev bit 7
    Rar, // A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0

    SubM, // A <- A + (HL)

    Xthl, // L <-> (SP); H <-> (SP+1)

    LdaxD, // A <- (DE)
    DcxD, // DE = DE-1

    Di, // special
    Unused, // ???
}

fn table() -> HashMap<&'static str, OpCmd> {
    let mut map = HashMap::new();
    map.insert("0x0", OpCmd::new(Cmd::Nop, ArgType::None));

    map.insert("0x3", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x8", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x10", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x18", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x20", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x28", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x30", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0x38", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0xCB", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0xD9", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0xDD", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0xED", OpCmd::new(Cmd::Unused, ArgType::None));
    map.insert("0xFD", OpCmd::new(Cmd::Unused, ArgType::None));

    map.insert("0x96", OpCmd::new(Cmd::SubM, ArgType::None));
    map.insert("0x19", OpCmd::new(Cmd::DadD, ArgType::None));
    map.insert("0x1B", OpCmd::new(Cmd::DcxD, ArgType::None));


    map.insert("0x1", OpCmd::new(Cmd::LxiB, ArgType::ArgTwo));
    map.insert("0x11", OpCmd::new(Cmd::LxiD, ArgType::ArgTwo));
    map.insert("0x21", OpCmd::new(Cmd::LxiH, ArgType::ArgTwo));
    map.insert("0x31", OpCmd::new(Cmd::LxiSp, ArgType::ArgTwo));

    map.insert("0xE9", OpCmd::new(Cmd::Pchl, ArgType::None));

    map.insert("0x2", OpCmd::new(Cmd::StaxB, ArgType::None));
    map.insert("0x12", OpCmd::new(Cmd::StaxD, ArgType::None));

    map.insert("0x22", OpCmd::new(Cmd::Shld, ArgType::Address));

    map.insert("0x1A", OpCmd::new(Cmd::LdaxD, ArgType::None));

    map.insert("0x3E", OpCmd::new(Cmd::MviA, ArgType::ArgOne));
    map.insert("0x16", OpCmd::new(Cmd::MviD, ArgType::ArgOne));
    map.insert("0x1E", OpCmd::new(Cmd::MviE, ArgType::ArgOne));
    map.insert("0x2E", OpCmd::new(Cmd::MviL, ArgType::ArgOne));

    map.insert("0x80", OpCmd::new(Cmd::AddB, ArgType::None));
    map.insert("0x81", OpCmd::new(Cmd::AddC, ArgType::None));

    map.insert("0x15", OpCmd::new(Cmd::DcrD, ArgType::None));
    map.insert("0x1D", OpCmd::new(Cmd::DcrE, ArgType::None));
    map.insert("0x25", OpCmd::new(Cmd::DcrH, ArgType::None));

    map.insert("0xC3", OpCmd::new(Cmd::Jmp, ArgType::Address));
    map.insert("0xFA", OpCmd::new(Cmd::Jm, ArgType::Address));

    map.insert("0xC5", OpCmd::new(Cmd::PushB, ArgType::None));
    map.insert("0xD5", OpCmd::new(Cmd::PushD, ArgType::None));
    map.insert("0xE5", OpCmd::new(Cmd::PushH, ArgType::None));
    map.insert("0xF5", OpCmd::new(Cmd::PushPsw, ArgType::None));

    map.insert("0x7A", OpCmd::new(Cmd::MovAD, ArgType::None));

    map.insert("0x4F", OpCmd::new(Cmd::MovCA, ArgType::None));
    map.insert("0x48", OpCmd::new(Cmd::MovCB, ArgType::None));
    map.insert("0x49", OpCmd::new(Cmd::MovCC, ArgType::None));
    map.insert("0x4A", OpCmd::new(Cmd::MovCD, ArgType::None));
    map.insert("0x4B", OpCmd::new(Cmd::MovCE, ArgType::None));
    map.insert("0x4C", OpCmd::new(Cmd::MovCH, ArgType::None));
    map.insert("0x4D", OpCmd::new(Cmd::MovCL, ArgType::None));
    map.insert("0x4E", OpCmd::new(Cmd::MovCM, ArgType::None));

    map.insert("0x67", OpCmd::new(Cmd::MovHA, ArgType::None));
    map.insert("0x66", OpCmd::new(Cmd::MovHM, ArgType::None));

    map.insert("0xE4", OpCmd::new(Cmd::Cpo, ArgType::Address));

    map.insert("0xCA", OpCmd::new(Cmd::Jz, ArgType::Address));
    map.insert("0xDA", OpCmd::new(Cmd::Jc, ArgType::Address));
    map.insert("0xC2", OpCmd::new(Cmd::Jnz, ArgType::Address));

    map.insert("0x99", OpCmd::new(Cmd::SbbC, ArgType::None));

    map.insert("0x17", OpCmd::new(Cmd::Ral, ArgType::None));
    map.insert("0x1F", OpCmd::new(Cmd::Rar, ArgType::None));

    map.insert("0x1C", OpCmd::new(Cmd::InrE, ArgType::None));
    map.insert("0x14", OpCmd::new(Cmd::InrD, ArgType::None));
    map.insert("0x24", OpCmd::new(Cmd::InrH, ArgType::None));

    map.insert("0x13", OpCmd::new(Cmd::InxD, ArgType::None));
    map.insert("0x23", OpCmd::new(Cmd::InxH, ArgType::None));

    map.insert("0xFF", OpCmd::new(Cmd::Rst7, ArgType::None));
    map.insert("0xC8", OpCmd::new(Cmd::Rz, ArgType::None));

    map.insert("0xE3", OpCmd::new(Cmd::Xthl, ArgType::None));

    map.insert("0xF3", OpCmd::new(Cmd::Di, ArgType::None));
    map
}