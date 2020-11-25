use crate::RandomNumber;
use crate::RandomNumberInit;
use crate::split_mix::SplitMix;

mod constants {
    // the paper (https://arxiv.org/pdf/1704.00358v5.pdf) says any potential issues with non-randomness are gone after ~5 iterations
    // let's do 10 here to be safe
    pub const INIT_CALLS: u64 = 10;

    pub const S: u64 = 0x9f32e1cbc5e1374b;
}

#[derive(Debug)]
pub struct MiddleSquaresWeylSequence {
    x: u64,
    w: u64
}

impl RandomNumberInit for MiddleSquaresWeylSequence {
    fn new() -> MiddleSquaresWeylSequence {
        msws_from_split_mix(&mut SplitMix::new())
    }

    fn from_seed(seed: u64) -> MiddleSquaresWeylSequence {
        msws_from_split_mix(&mut SplitMix::from_seed(seed))
    }
}

impl RandomNumber for MiddleSquaresWeylSequence {
    #[inline]
    fn next_bool(&mut self) -> bool {
        self.next_u32() >= crate::TWO_31
    }

    #[inline]
    fn next_u8(&mut self) -> u8 {
        (self.next_u32() >> 24) as u8
    }

    #[inline]
    fn next_u16(&mut self) -> u16 {
        (self.next_u32() >> 16) as u16
    }

    fn next_u32(&mut self) -> u32 {
        update_state(self);
        
        extract_middle(self);

        self.x as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }

    fn next_f64(&mut self) -> f64 {
        update_state(self);

        let xx = self.x;

        extract_middle(self);

        update_state(self);

        extract_middle(self);

        (((xx ^ self.x) >> 11) as f64) * crate::TWO_53_INVERSE
    }
}

fn msws_from_split_mix(split_mix: &mut SplitMix) -> MiddleSquaresWeylSequence {
    let mut msws = MiddleSquaresWeylSequence {
        x: split_mix.next_u64(),
        w: split_mix.next_u64()
    };

    init_msws(&mut msws);

    msws
}

fn init_msws(msws: &mut MiddleSquaresWeylSequence) {
    for _ in 0..constants::INIT_CALLS {
        msws.next_u64();

        msws.next_f64();
    }
}

fn update_state(msws: &mut MiddleSquaresWeylSequence) {
    msws.x = msws.x.wrapping_mul(msws.x);
    msws.w = msws.w.wrapping_add(constants::S);
    msws.x = msws.x.wrapping_add(msws.w);
}

fn extract_middle(msws: &mut MiddleSquaresWeylSequence) {
    msws.x = (msws.x >> 32) | (msws.x << 32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let mut msws = MiddleSquaresWeylSequence::new();

        msws.next_u64();
    }

    #[test]
    fn from_seed_test() {
        let msws = MiddleSquaresWeylSequence::from_seed(0);

        assert_eq!(msws.x, 8745858294955777127);
        assert_eq!(msws.w, 13984785221142081895);
    }

    #[test]
    fn next_u32_test() {
        let mut msws = MiddleSquaresWeylSequence::from_seed(0);

        assert_eq!(msws.next_u32(), 1651175586);
    }

    #[test]
    fn next_u64_test() {
        let mut msws = MiddleSquaresWeylSequence::from_seed(0);

        assert_eq!(msws.next_u64(), 7091745142799299804);
    }

    #[test]
    fn next_f64_test() {
        let mut msws = MiddleSquaresWeylSequence::from_seed(0);

        assert_eq!(msws.next_f64(), 0.250060447322643);
    }
}
