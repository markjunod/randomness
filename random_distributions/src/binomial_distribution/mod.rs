// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

// TODO add documentation

use std::fmt::Debug;

use crate::prelude::*;

#[derive(Debug)]
pub struct Binomial {
    random_number: Box<dyn RandomNumber>,
    success_probability: f64
}

#[derive(Debug)]
pub struct BinomialParams {
    num_trials: u64
}

impl DistributionParams for BinomialParams {}

// TODO add error handling
pub enum BinomialError {
    SuccessProbabiliyOutOfRange,
    NumTrialsTooSmall,
}

impl Binomial {
     pub fn new(random_algorithm: RandomNumberAlgorithm, success_probability: f64) -> Binomial {
        Binomial {
            random_number: random_numbers::new(random_algorithm),
            success_probability: success_probability
        }
    }

    pub fn from_seed(random_algorithm: RandomNumberAlgorithm, success_probability: f64, seed: u64) -> Binomial {
        Binomial {
            random_number: random_numbers::from_seed(random_algorithm, seed),
            success_probability: success_probability
        }
    }
}

impl SampleDistribution<bool> for Binomial {
    #[inline]
    fn sample(&mut self) -> bool {
        self.random_number.next_f64() < self.success_probability
    }
}

impl DiscreteDistribution<BinomialParams> for Binomial {
    fn pdf(&mut self, k: u64, params: &BinomialParams) -> f64 {
        let k_f64 = k as f64;

        let ln_p = ln_n_choose_k(params.num_trials, k) + self.success_probability.ln() * k_f64 + (1.0 - self.success_probability).ln() * k_f64;

        ln_p.exp()
    }
}

// n choose k = n! / k! * (n - k)!
// ln(n choose k) = ln(n!) - (ln(k!) + ln((n - k)!))
fn ln_n_choose_k(n: u64, k: u64) -> f64 {
    ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k)
}

// Ex: ln(3!) = ln(3*2*1) = ln(3)+ln(2)+ln(1)
fn ln_factorial(n: u64) -> f64 {
    (n..0).fold(0f64, |sum, next| (next as f64).ln() + sum)
}
