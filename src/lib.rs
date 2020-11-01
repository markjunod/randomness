use std::time::{SystemTime, UNIX_EPOCH};

mod mersenne_twister;
mod split_mix;
mod xorshift;

pub trait RandomNumber {
    fn next_u64(&mut self) -> u64;
}

pub trait RandomNumberInit {
    fn new() -> Self;

    fn from_seed(seed: u64) -> Self;
}

pub enum RandomNumberAlgorithm {
    MersenneTwister,
    XorshiftPlus,
    XoshiroSS
}

pub fn new_default() -> impl RandomNumber {
    mersenne_twister::MersenneTwister::new()
}

pub fn new(algorithm: RandomNumberAlgorithm) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(mersenne_twister::MersenneTwister::new()),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(xorshift::xorshift_plus::XorshiftPlus::new()),
        RandomNumberAlgorithm::XoshiroSS => Box::new(xorshift::xoshiro::XoshiroSS::new()),
    }
}

pub fn from_seed_default(seed: u64) -> impl RandomNumber {
    mersenne_twister::MersenneTwister::from_seed(seed)
}

pub fn from_seed(algorithm: RandomNumberAlgorithm, seed: u64) -> Box<dyn RandomNumber> {
    match algorithm {
        RandomNumberAlgorithm::MersenneTwister => Box::new(mersenne_twister::MersenneTwister::from_seed(seed)),
        RandomNumberAlgorithm::XorshiftPlus => Box::new(xorshift::xorshift_plus::XorshiftPlus::from_seed(seed)),
        RandomNumberAlgorithm::XoshiroSS => Box::new(xorshift::xoshiro::XoshiroSS::from_seed(seed)),
    }
}

pub fn current_nanos() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_nanos_test() {
        // make sure current_nanos() doesn't panic
        current_nanos();
    }
}
