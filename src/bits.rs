use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bits(pub u8);

impl Bits {
    pub fn bit(&self, offset: usize) -> u8 {
        debug_assert!(
            offset < 8,
            "argument 'offset' to Bits::bit cannot be greater than 7"
        );
        (self.0 & (1 << offset)) >> offset
    }

    pub fn bit_range(&self, range: Range<u8>) -> u8 {
        range.fold(0, |acc, offset| acc | (self.0 & (1 << offset)))
    }
}
