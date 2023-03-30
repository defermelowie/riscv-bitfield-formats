use std::mem::size_of;

pub mod base;
pub mod h_ext;

pub trait Csr {
    /// Create a new CSR instance from a value
    fn new(value: u64) -> Self
    where
        Self: Sized;

    /// Print CSR's value
    fn print(&self);
}

pub fn get_bit<I>(value: I, index: usize) -> I
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

pub fn get_bits<I>(value: I, start: usize, end: usize) -> I
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
    fn get_bit_u8_1() {
        assert_eq!(1, get_bit(0b0000_0010_u8, 1));
    }

    #[test]
    fn get_bit_u8_0() {
        assert_eq!(0, get_bit(0b1111_1011_u8, 2));
    }

    #[test]
    fn get_bit_u16_1() {
        assert_eq!(1, get_bit(0x4000_u16, 14));
    }

    #[test]
    fn get_bit_u16_0() {
        assert_eq!(0, get_bit(0xfdff_u16, 9));
    }

    #[test]
    fn get_bits_u16_l() {
        assert_eq!(3, get_bits(0b000_1100_u8, 2, 3))
    }

    #[test]
    fn get_bits_u16_m() {
        assert_eq!(0xf, get_bits(0b011_1100_u8, 2, 5))
    }
}