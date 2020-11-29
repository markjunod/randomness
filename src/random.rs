// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

//! Provides easy, high-level access to random primitive values, without needing to worry
//! about the underlying details (e.g. the [`Uniform`] distribution or the PRNG algorithm).
//! 
//! There is currently no access to random, non-primitive values.  You'll need to use the
//! [`Uniform`] distribution in order to have acess to random floats within a range other
//! than [0, 1].  Future work will provide access to things like random integers in a range,
//! random ASCII characters and strings, random elements of a collection, etc.

use random_distributions::prelude::*;

// macro to implement functions like `next_bool`
macro_rules! make_next {
    ($next_name: ident, $next_type: ty) => {
        #[inline]
        pub fn $next_name(&mut self) -> $next_type {
            (*self).uniform.sample()
        }
    };
}

/// Abstraction on top of the [`Uniform`] distribution to make it easy to access random
/// primitive values.  The distribution and PRNG algorithm are fixed and hidden on purpose.
/// 
/// If you need access to a specific PRNG algorith, or a non-uniform distribution, you'll
/// need to access the desired [`DiscreteDistribution`] or [`ContinuousDistribution`]
/// directly from the [`random_distributions`] library.
pub struct Random {
    uniform: Uniform
}

impl Random {
    /// Creates a new [`Random`] instance, seeded with the current time in nanos. Sufficient
    /// for most use cases.
    pub fn new() -> Random {
        Random {
            uniform: new_uniform(None)
        }
    }

    /// Creates a new [`Random`] instance with the given seed.  Useful when you need a repeatable
    /// "random" sequence of values.
    pub fn from_seed(seed: u64) -> Random {
        Random {
            uniform: from_seed_uniform(None, seed)
        }
    }
    
    make_next!(next_bool, bool);
    make_next!(next_u8, u8);
    make_next!(next_i8, i8);
    make_next!(next_u16, u16);
    make_next!(next_i16, i16);
    make_next!(next_u32, u32);
    make_next!(next_i32, i32);
    make_next!(next_u64, u64);
    make_next!(next_i64, i64);
    make_next!(next_u128, u128);
    make_next!(next_i128, i128);
    make_next!(next_f32, f32);
    make_next!(next_f64, f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_bool_test() {
        let mut random = Random::from_seed(0);

        assert!(random.next_bool());
    }

    #[test]
    fn next_u8_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_u8(), 222);
    }

    #[test]
    fn next_i8_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_i8(), -34);
    }

    #[test]
    fn next_u16_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_u16(), 57033);
    }

    #[test]
    fn next_i16_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_i16(), -8503);
    }

    #[test]
    fn next_u32_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_u32(), 3737718098);
    }

    #[test]
    fn next_i32_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_i32(), -557249198);
    }

    #[test]
    fn next_u64_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_u64(), 16053376993090331485);
    }

    #[test]
    fn next_i64_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_i64(), -2393367080619220131);
    }

    #[test]
    fn next_u128_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_u128(), 296132536910314333876124398186926965454);
    }

    #[test]
    fn next_i128_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_i128(), -44149830010624129587250209244841246002);
    }

    #[test]
    fn next_f32_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_f32(), 0.87025523);
    }

    #[test]
    fn next_f64_test() {
        let mut random = Random::from_seed(0);

        assert_eq!(random.next_f64(), 0.8702553105818676);
    }
}
