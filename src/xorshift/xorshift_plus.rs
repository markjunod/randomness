use crate::RandomNumber;
use crate::RandomNumberInit;
use crate::split_mix::SplitMix;

pub struct XorshiftPlus {
    a_state: u64,
    b_state: u64
}

impl RandomNumberInit for XorshiftPlus {
    fn new() -> XorshiftPlus {
        xorshift_plus_from_split_mix(&mut SplitMix::new())
    }

    fn from_seed(seed: u64) -> XorshiftPlus {
        xorshift_plus_from_split_mix(&mut SplitMix::from_seed(seed))
    }
}

impl RandomNumber for XorshiftPlus {
    fn next_u64(&mut self) -> u64 {
        let mut t = self.a_state;
        let s = self.b_state;

        t = t ^ (t << 23);
        t = t ^ (t >> 17);
        t = t ^ (s ^ (s >> 26));

        self.a_state = s;
        self.b_state = t;

        t.wrapping_add(s)
    }
}

fn xorshift_plus_from_split_mix(split_mix: &mut SplitMix) -> XorshiftPlus {
    XorshiftPlus {
        a_state: split_mix.next_u64(),
        b_state: split_mix.next_u64(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let mut xorshift = XorshiftPlus::new();

        xorshift.next_u64();
    }

    #[test]
    fn from_seed_test() {
        let xorshift = XorshiftPlus::from_seed(0);

        assert_eq!(xorshift.a_state, 0);
        assert_eq!(xorshift.b_state, 16294208416658607535);
    }

    #[test]
    fn next_u32_test() {
        let mut xorshift = XorshiftPlus::from_seed(0);

        assert_eq!(xorshift.next_u32(), 3292614715);
    }

    #[test]
    fn next_u64_test() {
        let mut xorshift = XorshiftPlus::from_seed(0);

        assert_eq!(xorshift.next_u64(), 14141672521104462240);
    }
}
