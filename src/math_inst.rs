use crate::{
    bits::Bits,
    error::*,
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
    Aci { imm: u8 },
    SubR { src: Register },
    SubM,
    Sui { imm: u8 },
    SbbR { src: Register },
    SbbM,
    Sbi { imm: u8 },
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
    fn parse(buf: &[u8; 3]) -> Result<Self> {
        use MathInst::*;
        match buf[0] {
            0b00_100_111 => return Ok(Daa),
            0b00_110_100 => return Ok(InrM),
            0b00_110_101 => return Ok(DcrM),
            0b10_000_110 => return Ok(AddM),
            0b10_001_110 => return Ok(AdcM),
            0b10_010_110 => return Ok(SubM),
            0b10_011_110 => return Ok(SbbM),
            0b11_000_110 => return Ok(Adi { imm: buf[1] }),
            0b11_001_110 => return Ok(Aci { imm: buf[1] }),
            0b11_010_110 => return Ok(Sui { imm: buf[1] }),
            0b11_011_110 => return Ok(Sbi { imm: buf[1] }),
            _ => (),
        };
        let bits = Bits::new(buf[0]);
        let (prefix, triple0, triple1) = (
            bits.bit_range(0..2),
            bits.bit_range(2..5),
            bits.bit_range(5..8),
        );
        match prefix {
            0b00 => match (triple0, triple1) {
                (dest, 0b100) => Some(InrR {
                    dest: dest.try_into()?,
                }),
                (dest, 0b101) => Some(DcrR {
                    dest: dest.try_into()?,
                }),
                _ => {
                    let rp = RegisterPair::try_from(bits.bit_range(2..4))?;
                    let discriminant = bits.bit_range(4..8);
                    match (rp, discriminant) {
                        (rp, 0b0011) => Some(Inx { rp }),
                        (rp, 0b1001) => Some(Dad { rp }),
                        (rp, 0b1011) => Some(Dcx { rp }),
                        _ => None,
                    }
                }
            },
            0b10 => match (triple0, triple1) {
                (0b000, src) => Some(AddR {
                    src: src.try_into()?,
                }),
                (0b001, src) => Some(AdcR {
                    src: src.try_into()?,
                }),
                (0b010, src) => Some(SubR {
                    src: src.try_into()?,
                }),
                (0b011, src) => Some(SbbR {
                    src: src.try_into()?,
                }),
                _ => None,
            },
            _ => None,
        }
        .ok_or(Error::IllegalInstruction(*buf))
    }

    fn size(&self) -> usize {
        use MathInst::*;
        match self {
            Adi { .. } | Aci { .. } | Sbi { .. } | Sui { .. } => 2,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_inst_eq, mkinst, mkinst_arr};

    #[test]
    fn test_math_parse() {
        use crate::register::{Register::*, RegisterPair::*};
        use MathInst::*;
        let cases = &[
            (mkinst_arr!(mkinst!(0b10_000, A)), AddR { src: A }),
            (mkinst_arr!(0b10_000_110), AddM),
            (mkinst_arr!(0b11_000_110, 0xFF), Adi { imm: 0xFF }),
            (mkinst_arr!(mkinst!(0b10_001, B)), AdcR { src: B }),
            (mkinst_arr!(0b10_001_110), AdcM),
            (mkinst_arr!(0b11_001_110, 0xAA), Aci { imm: 0xAA }),
            (mkinst_arr!(mkinst!(0b10_010, D)), SubR { src: D }),
            (mkinst_arr!(0b10_010_110), SubM),
            (mkinst_arr!(0b11_010_110, 0xAF), Sui { imm: 0xAF }),
            (mkinst_arr!(mkinst!(0b10_011, C)), SbbR { src: C }),
            (mkinst_arr!(0b10_011_110), SbbM),
            (mkinst_arr!(0b11_011_110, 0xCC), Sbi { imm: 0xCC }),
            (mkinst_arr!(mkinst!(0b00, B, 0b100)), InrR { dest: B }),
            (mkinst_arr!(0b00_110_100), InrM),
            (mkinst_arr!(mkinst!(0b00, A, 0b101)), DcrR { dest: A }),
            (mkinst_arr!(0b00_110_101), DcrM),
            (mkinst_arr!(mkinst!(0b00, rp = DE, 0b0011)), Inx { rp: DE }),
            (mkinst_arr!(mkinst!(0b00, rp = HL, 0b1011)), Dcx { rp: HL }),
            (mkinst_arr!(mkinst!(0b00, rp = DE, 0b1001)), Dad { rp: DE }),
            (mkinst_arr!(0b00_100_111), Daa),
        ];
        for (bytes, inst) in cases {
            assert_inst_eq!(MathInst::parse(bytes), bytes, inst);
        }
    }
}
