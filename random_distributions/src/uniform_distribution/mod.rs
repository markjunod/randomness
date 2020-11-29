// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

// TODO add documentation

use crate::prelude::*;

#[derive(Debug)]
pub struct Uniform {
    random_number: Box<dyn RandomNumber>
}

#[derive(Debug)]
pub struct DiscreteUniformParams {
    num_classes: u64
}

impl DistributionParams for DiscreteUniformParams {}

#[derive(Debug)]
pub struct ContinuousUniformParams {
    min: f64,
    max: f64
}

impl DistributionParams for ContinuousUniformParams {}

impl Uniform {
     pub fn new(random_algorithm: RandomNumberAlgorithm) -> Uniform {
        Uniform {
            random_number: random_numbers::new(random_algorithm)
        }
    }

    pub fn from_seed(random_algorithm: RandomNumberAlgorithm, seed: u64) -> Uniform {
        Uniform {
            random_number: random_numbers::from_seed(random_algorithm, seed)
        }
    }
}

macro_rules! make_sample_dist {
    ($next_name: ident, $sample_type: ty) => {
        impl SampleDistribution<$sample_type> for Uniform {
            #[inline]
            fn sample(&mut self) -> $sample_type {
                self.random_number.$next_name()
            }
        }
    };
}

make_sample_dist!(next_bool, bool);
make_sample_dist!(next_u8, u8);
make_sample_dist!(next_i8, i8);
make_sample_dist!(next_u16, u16);
make_sample_dist!(next_i16, i16);
make_sample_dist!(next_u32, u32);
make_sample_dist!(next_i32, i32);
make_sample_dist!(next_u64, u64);
make_sample_dist!(next_i64, i64);
make_sample_dist!(next_u128, u128);
make_sample_dist!(next_i128, i128);
make_sample_dist!(next_f32, f32);
make_sample_dist!(next_f64, f64);

impl Uniform {
    pub fn sample_f32_range(&mut self, min: f32, max: f32) -> f32 {
        let f: f32 = self.sample();

        f * (max - min) + min
    }

    pub fn sample_f64_range(&mut self, min: f64, max: f64) -> f64 {
        let f: f64 = self.sample();

        f * (max - min) + min
    }
}

impl DiscreteDistribution<DiscreteUniformParams> for Uniform {
    fn pdf(&mut self, _k: u64, params: &DiscreteUniformParams) -> f64 {
        1.0 / (params.num_classes as f64)
    }
}

impl ContinuousDistribution<ContinuousUniformParams> for Uniform {
    // the measure of a single point in any interval is always 0
    fn pdf(&mut self, _x: f64, _params: &ContinuousUniformParams) -> f64 {
        0.0
    }

    // TODO add error handling
    fn cdf(&mut self, x: f64, params: &ContinuousUniformParams) -> f64 {
        (x - params.min) / (params.max - params.min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_uniform_boolean_test() {
        let mut uniform = Uniform::from_seed(RandomNumberAlgorithm::Xoshiro256SS, 0);

        let b: bool = uniform.sample();

        assert!(b);
    }
}