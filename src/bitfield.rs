//! Defines a generic bitfield type

use std::{mem::size_of, fmt::Display};

/// A bitfield with 2 constant generics to indicate the position of it's first and last bit
///
/// Requires `START <= END`
pub struct BitField<const START: usize, const END: usize>(u64); // where {START <= END}

impl<const S: usize, const E: usize> BitField<S, E> {
    /// Create a new bitfield from a raw value
    /// 
    /// Value is taken from bits `START` to `END`
    pub fn new(value: u64) -> Self {
        assert!(S <= E);
        if Self::size() == 1 {
            BitField(get_bit(value.into(), S))
        } else {
            BitField(get_bits(value.into(), S, E))
        }
    }

    /// Get bitfield's size in bits
    pub fn size() -> usize {
        E - S + 1
    }

    /// Set bitfield's value
    ///
    /// Requires `value` to fit in bitfield's size
    pub fn set_value(&mut self, value: u64) {
        assert!(value < (1 << Self::size()));
        self.0 = value;
    }

    /// Get bitfield's value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<I, const S: usize, const E: usize> From<I> for BitField<S, E>
where
    I: Into<u64>,
{
    fn from(value: I) -> Self {
        BitField::new(value.into())
    }
}

impl <const S: usize, const E:usize> Display for BitField<S, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bit_str = format!("{:b}", &self.0);
        let zeroes = Self::size() - bit_str.len();
        for _ in 0..zeroes {
            bit_str = format!("0{}", bit_str);
        }
        write!(f, "0b{}", bit_str)
    }
}

/// Get the bit at a specified index
fn get_bit<I>(value: I, index: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= index);

    let value = value << (lastbit - index);
    let value = value >> (lastbit);
    value
}

/// Get the bits between 2 specified indices
fn get_bits<I>(value: I, start: usize, end: usize) -> I
where
    I: std::ops::Shl<usize, Output = I>,
    I: std::ops::Shr<usize, Output = I>,
{
    let lastbit = size_of::<I>() * 8 - 1;
    assert!(lastbit >= end);
    assert!(end >= start);

    let value = value << (lastbit - end);
    let value = value >> (lastbit - end + start);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn bitfield_start_after_end_panics() {
        let _: BitField<7, 4> = BitField::new(3);
    }

    #[test]
    fn bitfield_size() {
        let s = BitField::<4, 7>::size();
        assert_eq!(s, 4);
    }

    #[test]
    fn bitfield_from_u64() {
        let v = 0x4756_u64;
        let b: BitField<2, 5> = v.into();
        assert_eq!(b.value(), 5);
    }

    #[test]
    fn bitfield_format() {
        let b = BitField::<0, 5>::new(0x5);
        let s = format!("{}", b);
        assert_eq!(s, "0b000101");
    }
    #[test]
    fn get_bit_u8() {
        assert_eq!(1, get_bit(0b0000_0010_u8, 1));
        assert_eq!(0, get_bit(0b1111_1011_u8, 2));
    }

    #[test]
    fn get_bit_u16() {
        assert_eq!(1, get_bit(0x4000_u16, 14));
        assert_eq!(0, get_bit(0xfdff_u16, 9));
    }

    #[test]
    fn get_bits_u16() {
        assert_eq!(3, get_bits(0b000_1100_u8, 2, 3));
        assert_eq!(0xf, get_bits(0b011_1100_u8, 2, 5));
    }
}
