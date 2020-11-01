pub const W: u64 = 64; // number of bits MT is getting implemented for
pub const N: usize = 312; // size of the MT array

pub const M: usize = 156;
pub const R: u64 = 31;

pub const LOWER_MASK: u64 = (1 << R) - 1; // the binary number of r 1's
pub const UPPER_MASK: u64 = !LOWER_MASK;

pub const A: u64 = 0xB5026F5AA96619E9;

pub const U: u64 = 29;
pub const D: u64 = 0x5555555555555555;

pub const S: u64 = 17;
pub const B: u64 = 0x71D67FFFEDA60000;

pub const T: u64 = 37;
pub const C: u64 = 0xFFF7EEE000000000;

pub const L: u64 = 43;

pub const F: u64 = 6364136223846793005;