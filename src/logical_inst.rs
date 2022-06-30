use crate::{bits::Bits, error::*, instruction::Instruction, register::Register};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalInst {
    AnaR { src: Register },
    AnaM,
    Ani { imm: u8 },
    XraR { src: Register },
    XraM,
    Xri { imm: u8 },
    OraR { src: Register },
    OraM,
    Ori { imm: u8 },
    CmpR { src: Register },
    CmpM,
    Cpi { imm: u8 },
    Rlc,
    Rrc,
    Ral,
    Rar,
    Cma,
    Cmc,
    Stc,
}

impl Instruction for LogicalInst {
    fn parse(buf: &[u8; 3]) -> Result<Self> {
        use LogicalInst::*;
        if let Some(exact) = {
            match buf[0] {
                0xA6 => Some(AnaM),
                0xE6 => Some(Ani { imm: buf[1] }),
                0xAE => Some(XraM),
                0xEE => Some(Xri { imm: buf[1] }),
                0xB6 => Some(OraM),
                0xF6 => Some(Ori { imm: buf[1] }),
                0xBE => Some(CmpM),
                0xFE => Some(Cpi { imm: buf[1] }),
                0x07 => Some(Rlc),
                0x0F => Some(Rrc),
                0x17 => Some(Ral),
                0x1F => Some(Rar),
                0x2F => Some(Cma),
                0x3F => Some(Cmc),
                0x37 => Some(Stc),
                _ => None,
            }
        } {
            return Ok(exact);
        }
        let bits = Bits::new(buf[0]);
        let (prefix, triple0, triple1) = (
            bits.bit_range(0..2),
            bits.bit_range(2..5),
            bits.bit_range(5..8),
        );
        if prefix != 0b10 {
            return Err(Error::IllegalInstruction(*buf));
        }
        match triple0 {
            0b100 => Ok(AnaR {
                src: Register::try_from(triple1)?,
            }),
            0b101 => Ok(XraR {
                src: Register::try_from(triple1)?,
            }),
            0b110 => Ok(OraR {
                src: Register::try_from(triple1)?,
            }),
            0b111 => Ok(CmpR {
                src: Register::try_from(triple1)?,
            }),
            _ => Err(Error::IllegalInstruction(*buf)),
        }
    }

    fn size(&self) -> usize {
        use LogicalInst::*;
        match self {
            Ani { .. } | Xri { .. } | Ori { .. } | Cpi { .. } => 2,
            _ => 1,
        }
    }
}
