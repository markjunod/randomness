use random_numbers::prelude::*;

mod uniform;

#[test]
fn default_is_uniform() {
    let mut rand = random_numbers::new_default();

    assert!(uniform::is_random_number_uniform(&mut rand, None).is_ok());
}

#[test]
fn mersenne_twister_is_uniform() {
    let mut rand = random_numbers::new(RandomNumberAlgorithm::MersenneTwister);

    assert!(uniform::is_random_number_uniform(&mut *rand, None).is_ok());
}

#[test]
fn middle_square_weyl_sequence_is_uniform() {
    let mut rand = random_numbers::new(RandomNumberAlgorithm::MiddleSquareWeylSequence);

    assert!(uniform::is_random_number_uniform(&mut *rand, None).is_ok());
}

#[test]
fn xorhift_plus_is_uniform() {
    let mut rand = random_numbers::new(RandomNumberAlgorithm::XorshiftPlus);

    assert!(uniform::is_random_number_uniform(&mut *rand, None).is_ok());
}

#[test]
fn xoshiro_256ss_is_uniform() {
    let mut rand = random_numbers::new(RandomNumberAlgorithm::Xoshiro256SS);

    assert!(uniform::is_random_number_uniform(&mut *rand, None).is_ok());
}
