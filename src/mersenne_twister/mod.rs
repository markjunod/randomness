use crate::RandomNumber;
use crate::RandomNumberInit;

mod constants;

pub struct MersenneTwister {
    index: usize,
    mt_array: [u64; constants::N]
}

impl RandomNumberInit for MersenneTwister {
    fn new() -> MersenneTwister {
        let seed = crate::current_nanos();
        let mut mta = [0; constants::N];

        init_mt_array(seed, &mut mta);
        twist_mt_array(&mut mta);

        MersenneTwister {
            index: 0,
            mt_array: mta,
        }
    }
    
    fn from_seed(seed: u64) -> MersenneTwister {
        let mut mta = [0; constants::N];

        init_mt_array(seed, &mut mta);
        twist_mt_array(&mut mta);

        MersenneTwister {
            index: 0,
            mt_array: mta,
        }
    }
}

impl RandomNumber for MersenneTwister {
    fn next_u64(&mut self) -> u64 {
        if self.index >= constants::N {
            twist_mt_array(&mut self.mt_array);
            self.index = 0;
        }

        let y = self.mt_array[self.index];
        self.index += 1;
        
        temper(y)
    }
}

fn init_mt_array(seed: u64, mt_array: &mut [u64; constants::N]) {
    mt_array[0] = seed;
    for idx in 1..(constants::N - 1) {
        mt_array[idx] = constants::F.wrapping_mul(mt_array[idx - 1] ^ (mt_array[idx - 1] >> (constants::W - 2))) + (idx as u64);
    }
}

fn twist_mt_array(mt_array: &mut [u64; constants::N]) {
    for idx in 0..(constants::N - 1) {
        let x = (mt_array[idx] & constants::UPPER_MASK) + (mt_array[(idx + 1) % constants::N] & constants::LOWER_MASK);
        let mut x_a = x >> 1;
        if x % 2 == 1 {
            x_a = x_a ^ constants::A;
        }

        mt_array[idx] = mt_array[(idx + constants::M) % constants::N] ^ x_a;
    }
}

fn temper(y: u64) -> u64 {
    let mut t = y ^ ((y >> constants::U) & constants::D);
    t = t ^ ((t << constants::S) & constants::B);
    t = t ^ ((t << constants::T) & constants::C);
    t ^ (t >> constants::L)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_mt_random_number_no_seed() {
        let mut mt_rnd = MersenneTwister::new();

        for _ in 0..(constants::N * 4) {
            // make sure we can update the inner array multiple times without panicking
            mt_rnd.next_u64();
        }
    }

    #[test]
    fn new_mt_random_number_0_seed() {
        let mut mt_rnd = MersenneTwister::from_seed(0);

        for _ in 0..(constants::N * 4) {
            // make sure we can update the inner array multiple times without panicking
            mt_rnd.next_u64();
        }
    }

    #[test]
    fn init_mt_array_test() {
        let mut mt_array: [u64; constants::N] = [1; constants::N];
        init_mt_array(0, &mut mt_array);
        assert_eq!(mt_array[0], 0);
        assert_eq!(mt_array[1], 1);
    }

    #[test]
    fn twist_mt_array_test() {
        let mut mt_array: [u64; constants::N] = [1; constants::N];
        twist_mt_array(&mut mt_array);
    }

    #[test]
    fn temper_test() {
        assert_eq!(temper(1), 18014535948568577);
    }
}
