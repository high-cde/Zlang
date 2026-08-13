#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    PushConst = 0, Pop = 1, StoreGlobal = 2, LoadGlobal = 3,
    Add = 4, Sub = 5, Less = 6, Greater = 7, Eq = 8,
    JmpIfFalse = 9, Jmp = 10, SysCall = 11, Ret = 12, Nop = 13,
}
impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::PushConst, 1 => OpCode::Pop, 2 => OpCode::StoreGlobal, 3 => OpCode::LoadGlobal,
            4 => OpCode::Add, 5 => OpCode::Sub, 6 => OpCode::Less, 7 => OpCode::Greater, 8 => OpCode::Eq,
            9 => OpCode::JmpIfFalse, 10 => OpCode::Jmp, 11 => OpCode::SysCall, 12 => OpCode::Ret,
            _ => OpCode::Nop,
        }
    }
}
