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

impl SampleDistribution<bool> for Uniform {
    #[inline]
    fn sample(&mut self) -> bool {
        self.random_number.next_bool()
    }
}

impl SampleDistribution<u8> for Uniform {
    #[inline]
    fn sample(&mut self) -> u8 {
        self.random_number.next_u8()
    }
}

impl SampleDistribution<i8> for Uniform {
    #[inline]
    fn sample(&mut self) -> i8 {
        self.random_number.next_i8()
    }
}

impl SampleDistribution<u16> for Uniform {
    #[inline]
    fn sample(&mut self) -> u16 {
        self.random_number.next_u16()
    }
}

impl SampleDistribution<i16> for Uniform {
    #[inline]
    fn sample(&mut self) -> i16 {
        self.random_number.next_i16()
    }
}

impl SampleDistribution<u32> for Uniform {
    #[inline]
    fn sample(&mut self) -> u32 {
        self.random_number.next_u32()
    }
}

impl SampleDistribution<i32> for Uniform {
    #[inline]
    fn sample(&mut self) -> i32 {
        self.random_number.next_i32()
    }
}

impl SampleDistribution<u64> for Uniform {
    #[inline]
    fn sample(&mut self) -> u64 {
        self.random_number.next_u64()
    }
}

impl SampleDistribution<i64> for Uniform {
    #[inline]
    fn sample(&mut self) -> i64 {
        self.random_number.next_i64()
    }
}

impl SampleDistribution<u128> for Uniform {
    #[inline]
    fn sample(&mut self) -> u128 {
        self.random_number.next_u128()
    }
}

impl SampleDistribution<i128> for Uniform {
    #[inline]
    fn sample(&mut self) -> i128 {
        self.random_number.next_i128()
    }
}

impl SampleDistribution<usize> for Uniform {
    #[inline]
    fn sample(&mut self) -> usize {
        self.random_number.next_usize()
    }
}

impl SampleDistribution<f32> for Uniform {
    #[inline]
    fn sample(&mut self) -> f32 {
        self.random_number.next_f32()
    }
}

impl SampleDistribution<f64> for Uniform {
    #[inline]
    fn sample(&mut self) -> f64 {
        self.random_number.next_f64()
    }
}

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