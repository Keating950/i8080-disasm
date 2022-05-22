use crate::error::Result;

pub trait Instruction: Sized + Clone + Copy + PartialEq + Eq {
    fn parse(buf: &[u8; 3]) -> Result<Self>;
    fn size(&self) -> usize;
}
