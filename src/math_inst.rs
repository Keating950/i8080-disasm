use crate::{
    instruction::Instruction,
    register::{Register, RegisterPair},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathInst {
    AddR { src: Register },
    AddM,
    Adi { imm: u8 },
    AdcR { src: Register },
    AdcM,
    Aci,
    SubR { src: Register },
    SubM,
    Sui { imm: u8 },
    SbbR { src: Register },
    SbbM,
    Sbi { val: u8 },
    InrR { dest: Register },
    InrM,
    DcrR { dest: Register },
    DcrM,
    Inx { rp: RegisterPair },
    Dcx { rp: RegisterPair },
    Dad { rp: RegisterPair },
    Daa,
}

impl Instruction for MathInst {
    fn parse(buf: &[u8; 3]) -> crate::error::Result<Self> {
        todo!()
    }

    fn size(&self) -> usize {
        use MathInst::*;
        match self {
            Adi { .. } | Aci { .. } | Sbi { .. } => 2,
            _ => 1,
        }
    }
}
