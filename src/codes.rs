// pub fn table() -> HashMap<&'static str, OpCmd> {
//     let mut map = HashMap::new();
//     map.insert("0x0", OpCmd::new(Cmd::Nop, ArgType::None));

//     map.insert("0x3", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x8", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x10", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x18", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x20", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x28", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x30", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0x38", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0xCB", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0xD9", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0xDD", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0xED", OpCmd::new(Cmd::Unused, ArgType::None));
//     map.insert("0xFD", OpCmd::new(Cmd::Unused, ArgType::None));

//     // map.insert("0x96", OpCmd::new(Cmd::SubM, ArgType::None));
//     // map.insert("0x19", OpCmd::new(Cmd::DadD, ArgType::None));
//     // map.insert("0x1B", OpCmd::new(Cmd::DcxD, ArgType::None));


//     // map.insert("0x1", OpCmd::new(Cmd::LxiB, ArgType::ArgTwo));
//     // map.insert("0x11", OpCmd::new(Cmd::LxiD, ArgType::ArgTwo));
//     // map.insert("0x21", OpCmd::new(Cmd::LxiH, ArgType::ArgTwo));
//     // map.insert("0x31", OpCmd::new(Cmd::LxiSp, ArgType::ArgTwo));

//     // map.insert("0xE9", OpCmd::new(Cmd::Pchl, ArgType::None));

//     // map.insert("0x2", OpCmd::new(Cmd::StaxB, ArgType::None));
//     // map.insert("0x12", OpCmd::new(Cmd::StaxD, ArgType::None));

//     // map.insert("0x22", OpCmd::new(Cmd::Shld, ArgType::Address));

//     // map.insert("0x1A", OpCmd::new(Cmd::LdaxD, ArgType::None));

//     // map.insert("0x3E", OpCmd::new(Cmd::MviA, ArgType::ArgOne));
//     // map.insert("0x16", OpCmd::new(Cmd::MviD, ArgType::ArgOne));
//     // map.insert("0x1E", OpCmd::new(Cmd::MviE, ArgType::ArgOne));
//     // map.insert("0x2E", OpCmd::new(Cmd::MviL, ArgType::ArgOne));

//     // map.insert("0x80", OpCmd::new(Cmd::AddB, ArgType::None));
//     // map.insert("0x81", OpCmd::new(Cmd::AddC, ArgType::None));

//     // map.insert("0x15", OpCmd::new(Cmd::DcrD, ArgType::None));
//     // map.insert("0x1D", OpCmd::new(Cmd::DcrE, ArgType::None));
//     // map.insert("0x25", OpCmd::new(Cmd::DcrH, ArgType::None));

//     // map.insert("0xC3", OpCmd::new(Cmd::Jmp, ArgType::Address));
//     // map.insert("0xFA", OpCmd::new(Cmd::Jm, ArgType::Address));

//     // map.insert("0xC5", OpCmd::new(Cmd::PushB, ArgType::None));
//     // map.insert("0xD5", OpCmd::new(Cmd::PushD, ArgType::None));
//     // map.insert("0xE5", OpCmd::new(Cmd::PushH, ArgType::None));
//     // map.insert("0xF5", OpCmd::new(Cmd::PushPsw, ArgType::None));

//     // map.insert("0x7A", OpCmd::new(Cmd::MovAD, ArgType::None));

//     // map.insert("0x4F", OpCmd::new(Cmd::MovCA, ArgType::None));
//     // map.insert("0x48", OpCmd::new(Cmd::MovCB, ArgType::None));
//     // map.insert("0x49", OpCmd::new(Cmd::MovCC, ArgType::None));
//     // map.insert("0x4A", OpCmd::new(Cmd::MovCD, ArgType::None));
//     // map.insert("0x4B", OpCmd::new(Cmd::MovCE, ArgType::None));
//     // map.insert("0x4C", OpCmd::new(Cmd::MovCH, ArgType::None));
//     // map.insert("0x4D", OpCmd::new(Cmd::MovCL, ArgType::None));
//     // map.insert("0x4E", OpCmd::new(Cmd::MovCM, ArgType::None));

//     // map.insert("0x67", OpCmd::new(Cmd::MovHA, ArgType::None));
//     // map.insert("0x66", OpCmd::new(Cmd::MovHM, ArgType::None));

//     // map.insert("0xE4", OpCmd::new(Cmd::Cpo, ArgType::Address));

//     // map.insert("0xCA", OpCmd::new(Cmd::Jz, ArgType::Address));
//     // map.insert("0xDA", OpCmd::new(Cmd::Jc, ArgType::Address));
//     // map.insert("0xC2", OpCmd::new(Cmd::Jnz, ArgType::Address));

//     // map.insert("0x99", OpCmd::new(Cmd::SbbC, ArgType::None));

//     // map.insert("0x17", OpCmd::new(Cmd::Ral, ArgType::None));
//     // map.insert("0x1F", OpCmd::new(Cmd::Rar, ArgType::None));

//     // map.insert("0x1C", OpCmd::new(Cmd::InrE, ArgType::None));
//     // map.insert("0x14", OpCmd::new(Cmd::InrD, ArgType::None));
//     // map.insert("0x24", OpCmd::new(Cmd::InrH, ArgType::None));

//     // map.insert("0x13", OpCmd::new(Cmd::InxD, ArgType::None));
//     // map.insert("0x23", OpCmd::new(Cmd::InxH, ArgType::None));

//     // map.insert("0xFF", OpCmd::new(Cmd::Rst7, ArgType::None));
//     // map.insert("0xC8", OpCmd::new(Cmd::Rz, ArgType::None));

//     // map.insert("0xE3", OpCmd::new(Cmd::Xthl, ArgType::None));

//     // map.insert("0xF3", OpCmd::new(Cmd::Di, ArgType::None));
//     map
// }