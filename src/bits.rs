use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bits(u8);

impl Bits {
    pub const fn new(n: u8) -> Bits {
        Bits(n)
    }

    pub const fn bit(&self, n: u8) -> u8 {
        (self.0 & (1 << n)) >> n
    }

    pub fn bit_range(&self, range: Range<u8>) -> u8 {
        let end = 8 - range.end;
        let mut val = 0;
        for offset in range.rev().map(|n| 7 - n) {
            val |= self.0 & (1 << offset);
        }
        val >> end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        let bits = Bits(0b10110000);
        assert_eq!(bits.bit_range(0..2), 0b10);
    }
}
