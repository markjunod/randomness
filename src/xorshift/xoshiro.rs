use crate::RandomNumber;
use crate::RandomNumberInit;
use crate::split_mix::SplitMix;

pub struct XoshiroSS {
    state_array: [u64; 4]
}

impl RandomNumberInit for XoshiroSS {
    fn new() -> XoshiroSS {
        xoshiro_ss_from_split_mix(&mut SplitMix::new())
    }

    fn from_seed(seed: u64) -> XoshiroSS {
        xoshiro_ss_from_split_mix(&mut SplitMix::from_seed(seed))
    }
}

impl RandomNumber for XoshiroSS {
    fn next_u64(&mut self) -> u64 {
        let result = rol(self.state_array[1].wrapping_mul(5), 7).wrapping_mul(9);

        let t = self.state_array[1] << 17;
        
        self.state_array[2] = self.state_array[2] ^ self.state_array[0];
        self.state_array[3] = self.state_array[3] ^ self.state_array[1];
        self.state_array[1] = self.state_array[1] ^ self.state_array[2];
        self.state_array[0] = self.state_array[0] ^ self.state_array[3];

        self.state_array[2] = self.state_array[2] ^ t;
        self.state_array[3] = rol(self.state_array[3], 45);

        result
    }
}

fn xoshiro_ss_from_split_mix(split_mix: &mut SplitMix) -> XoshiroSS {
    XoshiroSS {
        state_array: [
            split_mix.next_u64(), 
            split_mix.next_u64(), 
            split_mix.next_u64(), 
            split_mix.next_u64()
        ],
    }
}

fn rol(n: u64, k: u64) -> u64 {
    (n << k) | (n >> (64 - k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let mut xoshiro = XoshiroSS::new();

        xoshiro.next_u64();
    }

    #[test]
    fn from_seed_test() {
        let mut xoshiro = XoshiroSS::from_seed(0);

        assert_eq!(xoshiro.state_array[0], 0);
        assert_eq!(xoshiro.state_array[1], 16294208416658607535);
        assert_eq!(xoshiro.state_array[2], 7960286522194355700);
        assert_eq!(xoshiro.state_array[3], 487617019471545679);

        assert_eq!(xoshiro.next_u64(), 16053376993090331485);
    }

    #[test]
    fn rol_test_0_1() {
        assert_eq!(rol(0, 1), 0);
    } 
}
