use crate::RandomNumber;
use crate::RandomNumberInit;

mod constants;

pub struct SplitMix {
    current_state: u64
}

impl RandomNumberInit for SplitMix {
    fn new() -> SplitMix {
        SplitMix {
            current_state: crate::current_nanos(),
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
    fn mix_test() {
        let mut split_mix = SplitMix::from_seed(0);

        let n = mix(&mut split_mix);

        assert_eq!(n, 0);
        assert_eq!(split_mix.current_state, constants::A);
    }
}
