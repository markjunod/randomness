// Copyright (c) 2020 Mark Junod. Subject to the MIT License

//! Traits defining pseudo-random number generation algorithms, and utility methods for getting specific
//! implementations of those traits.
//! 
//! # Using the Library
//! 
//! 

pub mod prelude {
    pub use crate::*;
    pub use crate::RandomNumber;
}

mod mersenne_twister;
mod msws;
mod seeds;
mod split_mix;
mod xorshift;

use mersenne_twister::MersenneTwister;
use msws::MiddleSquaresWeylSequence;
use xorshift::xorshift_plus::XorshiftPlus;
use xorshift::xoshiro_256ss::Xoshiro256SS;

/// 2^31. Primarily used to convert a random u32 into a bool. We compare the random u32 to 2^31 instead
/// of comparing it to 1 mod 2 because the highest bit is usually of better quality than the lowest bit.
pub const TWO_31: u32 = 0b10000000_00000000_00000000_00000000;

/// 2^63. Primarily used to convert a random u64 into a bool.  We compare the random u64 to 2^63 instead
/// of comparing it to 1 mod 2 because the highest bit is usually of better quality than the lowest bit.
pub const TWO_63: u64 = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

/// 2^32. Used to convert random 32-bit unsigned integers into f32s between 0 and 1.
pub const TWO_32: f32 = 4294967296.0;
pub const TWO_32_INVERSE: f32 = 1.0 / TWO_32;

/// 2^53. Used to convert random 64-bit unsigned integers into f64s between 0 and 1.
pub const TWO_53: f64 = 9007199254740992.0;
pub const TWO_53_INVERSE: f64 = 1.0 / TWO_53;

pub trait RandomNumberInit {
    fn new() -> Self;

    fn from_seed(seed: u64) -> Self;
}

pub trait RandomNumber {
    fn next_bool(&mut self) -> bool {
        self.next_u64() >= TWO_63
    }

    fn next_u8(&mut self) -> u8 {
        (self.next_u64() >> 56) as u8
    }

    fn next_i8(&mut self) -> i8 {
        self.next_u8() as i8
    }

    fn next_u16(&mut self) -> u16 {
        (self.next_u64() >> 48) as u16
    }

    fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    fn next_u64(&mut self) -> u64;

    fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    fn next_u128(&mut self) -> u128 {
        (self.next_u64() as u128) << 64 | (self.next_u64() as u128)
    }

    fn next_i128(&mut self) -> i128 {
        self.next_u128() as i128
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) * TWO_32_INVERSE
    }

    fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * TWO_53_INVERSE
    }
}

pub enum RandomNumberAlgorithm {
    MersenneTwister,
    MiddleSquareWeylSequence,
    XorshiftPlus,
    Xoshiro256SS
}

pub fn new_default() -> impl RandomNumber {
    Xoshiro256SS::new()
}

pub fn new(algorithm: RandomNumberAlgorithm) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(MersenneTwister::new()),
        RandomNumberAlgorithm::MiddleSquareWeylSequence => Box::new(MiddleSquaresWeylSequence::new()),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(XorshiftPlus::new()),
        RandomNumberAlgorithm::Xoshiro256SS => Box::new(Xoshiro256SS::new()),
    }
}

pub fn from_seed_default(seed: u64) -> impl RandomNumber {
    Xoshiro256SS::from_seed(seed)
}

pub fn from_seed(algorithm: RandomNumberAlgorithm, seed: u64) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(MersenneTwister::from_seed(seed)),
        RandomNumberAlgorithm::MiddleSquareWeylSequence => Box::new(MiddleSquaresWeylSequence::from_seed(seed)),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(XorshiftPlus::from_seed(seed)),
        RandomNumberAlgorithm::Xoshiro256SS => Box::new(Xoshiro256SS::from_seed(seed)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_delete() {
        let n: u32 = 1;
        assert_eq!(n as f32, 1.0);
    }
}
