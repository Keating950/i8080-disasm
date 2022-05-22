use crate::error::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    A = 0b111,
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
}

impl TryFrom<u8> for Register {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        use Register::*;
        const VALUES: [Register; 7] = [A, B, C, D, E, H, L];
        match VALUES.binary_search_by(|e| (*e as u8).cmp(&value)) {
            Ok(idx) => Ok(VALUES[idx]),
            Err(_) => Err(Error::InvalidRegister(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterPair {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl TryFrom<u8> for RegisterPair {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        use RegisterPair::*;
        match value {
            n if n == BC as u8 => Ok(BC),
            n if n == DE as u8 => Ok(DE),
            n if n == HL as u8 => Ok(HL),
            n if n == SP as u8 => Ok(SP),
            _ => Err(Error::InvalidRegisterPair(value)),
        }
    }
}
