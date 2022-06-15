use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalInstruction([u8; 3]),
    InvalidRegister(u8),
    InvalidRegisterPair(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            IllegalInstruction(bytes) => write!(
                f,
                "Illegal instruction: [{:03b}, {:03b}, {:03b}]",
                bytes[0], bytes[1], bytes[2]
            ),
            InvalidRegister(n) => write!(f, "Could not interpret {:03b} as a register", n),
            InvalidRegisterPair(n) => write!(f, "Could not interpret {:02b} as a register pair", n),
        }
    }
}

impl std::error::Error for Error {}
