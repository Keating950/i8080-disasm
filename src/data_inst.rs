use crate::{
    bits::Bits,
    error::*,
    instruction::Instruction,
    register::{Register, RegisterPair},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataInst {
    MovR {
        dest: Register,
        src: Register,
    },
    MovFromM {
        dest: Register,
    },
    MovToM {
        src: Register,
    },
    MviR {
        dest: Register,
        imm: u8,
    },
    MviM {
        imm: u8,
    },
    Lxi {
        dest: RegisterPair,
        imm0: u8,
        imm1: u8,
    },
    Lda {
        addr: u16,
    },
    Sta {
        addr: u16,
    },
    Lhld {
        addr: u16,
    },
    Shld {
        addr: u16,
    },
    Ldax {
        rp: RegisterPair,
    },
    Stax {
        rp: RegisterPair,
    },
    Xchg,
}

impl Instruction for DataInst {
    fn parse(buf: &[u8; 3]) -> Result<Self> {
        use DataInst::*;
        match buf[0] {
            0x36 => return Ok(MviM { imm: buf[1] }),
            0x3A => {
                return Ok(Lda {
                    addr: u16::from_le_bytes(buf[1..].try_into().unwrap()),
                });
            }
            0x32 => {
                return Ok(Sta {
                    addr: u16::from_le_bytes(buf[1..].try_into().unwrap()),
                });
            }
            0x2A => {
                return Ok(Lhld {
                    addr: u16::from_le_bytes(buf[1..].try_into().unwrap()),
                });
            }
            0x22 => {
                return Ok(Shld {
                    addr: u16::from_le_bytes(buf[1..].try_into().unwrap()),
                });
            }
            0xEB => return Ok(Xchg),
            _ => {}
        };
        let bits = Bits(buf[0]);
        match bits.bit_range(0..1) {
            // Immediate and RP instructions
            0b00 => match (bits.bit_range(2..5), bits.bit_range(5..8)) {
                (0b110, 0b110) => Ok(MviM { imm: buf[1] }),
                (0b111, 0b010) => Ok(Lda {
                    addr: u16::from_le_bytes(buf[1..2].try_into().unwrap()),
                }),
                (dest, 0b110) => Ok(MviR {
                    dest: dest.try_into()?,
                    imm: buf[1],
                }),
                (rp, 0b001) => Ok(Lxi {
                    dest: RegisterPair::try_from(rp >> 1)?,
                    imm0: buf[1],
                    imm1: buf[2],
                }),
                _ => {
                    let rp = RegisterPair::try_from(bits.bit_range(2..4))?;
                    let discriminant = bits.bit_range(4..8);
                    match discriminant {
                        0b1010 => Ok(Ldax { rp }),
                        0b0010 => Ok(Ldax { rp }),
                        _ => Err(Error::IllegalInstruction(buf.clone())),
                    }
                }
            },
            // Mov instructions
            0b01 => match (bits.bit_range(2..5), bits.bit_range(5..8)) {
                (dest, 0b110) => Ok(MovFromM {
                    dest: dest.try_into()?,
                }),
                (0b110, src) => Ok(MovToM {
                    src: src.try_into()?,
                }),
                (dest, src) => Ok(MovR {
                    dest: dest.try_into()?,
                    src: src.try_into()?,
                }),
            },
            _ => Err(Error::IllegalInstruction(buf.clone())),
        }
    }

    fn size(&self) -> usize {
        use DataInst::*;
        match self {
            MovR { .. } | MovFromM { .. } | MovToM { .. } | Ldax { .. } | Xchg => 1,
            MviR { .. } | MviM { .. } => 2,
            _ => 3,
        }
    }
}
