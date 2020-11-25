use crate::seeds;
use crate::RandomNumber;
use crate::RandomNumberInit;

mod constants{
    pub const A: u64 = 0x9E3779B97f4A7C15;

    pub const R: u64 = 30;
    pub const B: u64 = 0xBF58476D1CE4E5B9;

    pub const S: u64 = 27;
    pub const C: u64 = 0x94D049BB133111EB;

    pub const T: u64 = 31;
}

#[derive(Debug)]
pub struct SplitMix {
    current_state: u64
}

impl RandomNumberInit for SplitMix {
    fn new() -> SplitMix {
        SplitMix {
            current_state: seeds::current_nanos(),
        }
    }

    fn from_seed(seed: u64) -> SplitMix {
        SplitMix {
            current_state: seed,
        }
    }
}

impl RandomNumber for SplitMix {
    fn next_u64(&mut self) -> u64 {
        mix(self)
    }
}

fn mix(split_mix: &mut SplitMix) -> u64 {
    let mut n = split_mix.current_state;

    split_mix.current_state = n.wrapping_add(constants::A);

    n = (n ^ (n >> constants::R)).wrapping_mul(constants::B);
    n = (n ^ (n >> constants::S)).wrapping_mul(constants::C);
    n ^ (n >> constants::T)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let mut split_mix = SplitMix::new();

        split_mix.next_u64();
    }

    #[test]
    fn from_seed_test() {
        let mut split_mix = SplitMix::from_seed(0);

        assert_eq!(split_mix.current_state, 0);

        let n = split_mix.next_u64();

        assert_eq!(n, 0);
        assert_eq!(split_mix.current_state, constants::A);
    }

    #[test]
    fn next_u32_test() {
        let mut split_mix = SplitMix::from_seed(0);

        assert_eq!(split_mix.next_u32(), 0);
    }

    #[test]
    fn next_u64_test() {
        let mut split_mix = SplitMix::from_seed(0);

        assert_eq!(split_mix.next_u64(), 0);
    }

    #[test]
    fn mix_test() {
        let mut split_mix = SplitMix::from_seed(0);

        let n = mix(&mut split_mix);

        assert_eq!(n, 0);
        assert_eq!(split_mix.current_state, constants::A);
    }
}
