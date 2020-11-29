// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

//! Traits defining random distributions, and utility methods to make accessing specific
//! implementations and functionality easier.

// TODO add documentation

pub mod prelude {
    pub use crate::*;
    pub use crate::SampleDistribution;
    pub use crate::uniform_distribution::Uniform;
}

mod binomial_distribution;
// TODO add a normal distribution
mod uniform_distribution;

use std::ops::{Add, Sub, Mul};

use random_numbers::prelude::*;

use binomial_distribution::*;
use uniform_distribution::*;

pub trait DistributionParams {}

pub trait DiscreteDistribution<T: DistributionParams> {
    fn pdf(&mut self, k: u64, distribution_params: &T) -> f64;

    #[inline]
    fn cdf(&mut self, k: u64, distribution_params: &T) -> f64 {
        (0..k+1).fold(0f64, |sum, i| self.pdf(i, distribution_params) + sum)
    }
}

pub trait ContinuousDistribution<T: DistributionParams> {
    fn pdf(&mut self, x: f64, distribution_params: &T) -> f64;

    fn cdf(&mut self, x: f64, distribution_params: &T) -> f64;
}

/// Defines a random distribution over a type `T`. Not all types are implemented
/// for all distributions.  If a type is missing, you can extend the needed
/// distrubtion.
/// 
/// Calling `sample` will produce a new instance of `T`.
pub trait SampleDistribution<T: ?Sized> {
    fn sample(&mut self) -> T;
}

pub trait SampleIntervalDistribution<T: Ord + Add + Sub + Mul> {
    fn sample_interval(&mut self, min: T, max: T) -> T;
}

pub fn new_uniform(random_number_algorithm: Option<RandomNumberAlgorithm>) -> Uniform {
    match random_number_algorithm {
        Some(algorithm) => Uniform::new(algorithm),
        None => Uniform::new(DEFAULT_RANDOM_NUMBER_ALGORITHM),
    }
}

pub fn from_seed_uniform(random_number_algorithm: Option<RandomNumberAlgorithm>, seed: u64) -> Uniform {
    match random_number_algorithm {
        Some(algorithm) => Uniform::from_seed(algorithm, seed),
        None => Uniform::from_seed(DEFAULT_RANDOM_NUMBER_ALGORITHM, seed),
    }
}

pub fn new_binomial(random_number_algorithm: Option<RandomNumberAlgorithm>, success_probability: f64) -> Binomial {
    match random_number_algorithm {
        Some(algorithm) => Binomial::new(algorithm, success_probability),
        None => Binomial::new(DEFAULT_RANDOM_NUMBER_ALGORITHM, success_probability),
    }
}

pub fn from_seed_binomial(random_number_algorithm: Option<RandomNumberAlgorithm>, seed: u64, success_probability: f64) -> Binomial {
    match random_number_algorithm {
        Some(algorithm) => Binomial::from_seed(algorithm, success_probability, seed),
        None => Binomial::from_seed(DEFAULT_RANDOM_NUMBER_ALGORITHM, success_probability, seed),
    }
}
