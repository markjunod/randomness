// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

//! Traits defining pseudo-random number generation algorithms, and utility methods for getting specific
//! implementations of those traits.
//! 
//! # Using the Library
//! 
//! The easiest way to get started is to use the default random number generator with the default seed.
//! This will supply the implementation that currently (according to the subjective judgement of the
//! authors) is the best mix of randomness and speed:
//! 
//! ```
//! use random_numbers::prelude::*;
//! 
//! let mut rand = random_numbers::new_default();
//! 
//! if rand.next_bool() {
//!     println!("u64: {}", rand.next_u64());
//! }
//! ```
//! 
//! If you want to supply your own seed, you can:
//! 
//! ```
//! use random_numbers::prelude::*;
//! 
//! let my_seed: u64 = 0;
//! 
//! let mut rand = random_numbers::from_seed_default(my_seed);
//! ```
//! 
//! Finally, if you want to choose a specific algorithm, use the [`RandomNumberAlgorithm`] enum
//! to specify the algorithm:
//! 
//! ```
//! use random_numbers::prelude::*;
//! 
//! let mut rand_mt = random_numbers::from_seed(RandomNumberAlgorithm::MersenneTwister, 0);
//! ```
//! 
//! This is a low-level library and each numerical type (plus `bool`) has its own method.

/// A convenient way to import all the main functionality of the library.
/// Just use
/// 
/// ```
/// use random_numbers::prelude::*;
/// ```
/// 
/// and you can get started creating a new generator and using it to supply
/// random numbers.
pub mod prelude {
    pub use crate::*;
    pub use crate::RandomNumber;
}

mod mersenne_twister;
mod msws;
mod seeds;
mod split_mix;
mod xorshift;

use std::fmt::Debug;

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

/// 2^23. Used to convert random 32-bit unsigned integers into f32s between 0 and 1.
pub const TWO_23: f32 = 0b00000000_10000000_00000000_00000000u32 as f32;
pub const TWO_23_INVERSE: f32 = 1.0 / TWO_23;

/// 2^53. Used to convert random 64-bit unsigned integers into f64s between 0 and 1.
pub const TWO_53: f64 = 0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000u64 as f64;
pub const TWO_53_INVERSE: f64 = 1.0 / TWO_53;

/// Specifies how a random number generator should be able to be initialized.
pub trait RandomNumberInit {
    /// Creates a new random number generator, using the current time in nanos as its seed.
    /// 
    /// This should be sufficient in most situations where you don't need/want repeatable
    /// sequences of random numbers.
    /// 
    /// What it means to be a "seed" depends on the algorithm chosen.
    fn new() -> Self;

    /// Supply your own seed to the random number generator.  The same seed will always
    /// produce the same sequence of random numbers.  Useful if you want repeatable tests.
    /// 
    /// What it means to be a "seed" depends on the algorithm chosen.
    fn from_seed(seed: u64) -> Self;
}

/// Specifies all the types a random number generator implementation should be able to provide.
/// 
/// In general, the upper bits are at least as good as the lower bits, if not better.  For this
/// reason we generally take the upper n bits for the smaller scalar data types (e.g. [`u8`]).
/// 
/// However, any individual implementation can override this behavior if desired.
pub trait RandomNumber: Debug {
    /// Returns a random [`bool`].
    #[inline]
    fn next_bool(&mut self) -> bool {
        self.next_u64() >= TWO_63
    }

    /// Returns a random [`u8`].
    #[inline]
    fn next_u8(&mut self) -> u8 {
        (self.next_u64() >> 56) as u8
    }

    /// Returns a random [`i8`].
    #[inline]
    fn next_i8(&mut self) -> i8 {
        self.next_u8() as i8
    }

    /// Returns a random [`u16`].
    #[inline]
    fn next_u16(&mut self) -> u16 {
        (self.next_u64() >> 48) as u16
    }

    /// Returns a random [`i16`].
    #[inline]
    fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }

    /// Returns a random [`u32`].
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Returns a random [`i32`].
    #[inline]
    fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    /// Returns a random [`u64`].
    fn next_u64(&mut self) -> u64;

    /// Returns a random [`i64`].
    #[inline]
    fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    /// Returns a random [`u128`].
    #[inline]
    fn next_u128(&mut self) -> u128 {
        (self.next_u64() as u128) << 64 | (self.next_u64() as u128)
    }

    /// Returns a random [`i128`].
    #[inline]
    fn next_i128(&mut self) -> i128 {
        self.next_u128() as i128
    }

    /// Returns a random [`f32`].
    #[inline]
    fn next_f32(&mut self) -> f32 {
        ((self.next_u32() >> 9) as f32) * TWO_23_INVERSE
    }

    /// Returns a random [`f64`].
    #[inline]
    fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * TWO_53_INVERSE
    }

    /// Returns a random [`usize`] on 16-bit architectures
    #[inline]
    #[cfg(any(target_pointer_width = "16"))]
    fn next_usize(&mut self) -> usize {
        self.next_u16() as usize
    }

    /// Returns a random [`usize`] on 32-bit architectures
    #[inline]
    #[cfg(any(target_pointer_width = "32"))]
    fn next_usize(&mut self) -> usize {
        self.next_u32() as usize
    }

    /// Returns a random [`usize`] on 64-bit architectures
    #[inline]
    #[cfg(any(target_pointer_width = "64"))]
    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }
}

/// List of implemented Random Number Generators available for selection.
/// Use these in the [`new()`] or [`from_seed()`] methods to choose the
/// specific algorithm you want to use.
/// 
/// See [Wikipedia PRGN List](https://en.wikipedia.org/wiki/List_of_random_number_generators)
/// for a list of possible additions.
pub enum RandomNumberAlgorithm {
    /// See [the Mersenne Twister Wikipedia page](https://en.wikipedia.org/wiki/Mersenne_Twister).
    MersenneTwister,
    /// See [the Middle Square Weyl Sequence Wikipedia page](https://en.wikipedia.org/wiki/Middle-square_method#Middle_Square_Weyl_Sequence_PRNG).
    MiddleSquareWeylSequence,
    /// See [the Xorhsift Wikipedia page](https://en.wikipedia.org/wiki/Xorshift).
    XorshiftPlus,
    /// See [the Xorshift Wikipedia page](https://en.wikipedia.org/wiki/Xorshift).
    Xoshiro256SS
}

pub const DEFAULT_RANDOM_NUMBER_ALGORITHM: RandomNumberAlgorithm = RandomNumberAlgorithm::Xoshiro256SS;

/// Generates a new default random number generator seeded with the current time in nanos.
/// This is the easiest way to get started using the library.
pub fn new_default() -> Box<dyn RandomNumber> {
    new(DEFAULT_RANDOM_NUMBER_ALGORITHM)
}

/// Generates a new random number generator using the given implementation, seeded with
/// the current time in nanos.
pub fn new(algorithm: RandomNumberAlgorithm) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(MersenneTwister::new()),
        RandomNumberAlgorithm::MiddleSquareWeylSequence => Box::new(MiddleSquaresWeylSequence::new()),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(XorshiftPlus::new()),
        RandomNumberAlgorithm::Xoshiro256SS => Box::new(Xoshiro256SS::new()),
    }
}

/// Generates a new default random number generator, seeded with the given `u64`.
pub fn from_seed_default(seed: u64) -> Box<dyn RandomNumber> {
    from_seed(DEFAULT_RANDOM_NUMBER_ALGORITHM, seed)
}

/// Generates a new random number generator using the given implementation, seeded with
/// the given `u64`.
pub fn from_seed(algorithm: RandomNumberAlgorithm, seed: u64) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(MersenneTwister::from_seed(seed)),
        RandomNumberAlgorithm::MiddleSquareWeylSequence => Box::new(MiddleSquaresWeylSequence::from_seed(seed)),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(XorshiftPlus::from_seed(seed)),
        RandomNumberAlgorithm::Xoshiro256SS => Box::new(Xoshiro256SS::from_seed(seed)),
    }
}
