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
        let start = range.start;
        range.fold(0, |acc, offset| acc | (self.0 & (1 << offset))) >> start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_range() {
        let cases = [(0b11100100, (0, 4), 0b0100), (0b11100100, (4, 8), 0b1110)];
        for (val, (low, high), output) in cases {
            let bits = Bits(val);
            let bit_range = bits.bit_range(low..high);
            assert_eq!(
                bit_range, output,
                "\nInput:\t{:08b}\nRange:\t{}..{}\nLeft:\t{:04b}\nRight:\t{:04b}",
                val, low, high, bit_range, output
            )
        }
    }
}
