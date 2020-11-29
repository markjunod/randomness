use randomness::prelude::*;

#[test]
fn initialize_new_random_struct() {
    let mut rand = new_random();

    let b = rand.next_bool();

    // don't care about the value, just that it was successful
    assert!(b || !b);
}

#[test]
fn initialize_new_random_struct_from_seed() {
    let mut rand = new_random_from_seed(0);

    assert_eq!(rand.next_bool(), true);
    assert_eq!(rand.next_u8(), 192);
    assert_eq!(rand.next_i8(), 14);
    assert_eq!(rand.next_u16(), 57771);
    assert_eq!(rand.next_i16(), -22443);
    assert_eq!(rand.next_u32(), 2731305002);
    assert_eq!(rand.next_i32(), -250457456);
    assert_eq!(rand.next_u64(), 13256358885486924480);
    assert_eq!(rand.next_i64(), 6862501136150115846);
    assert_eq!(rand.next_u128(), 132999424324388957102843903434974122987);
    assert_eq!(rand.next_i128(), 3612381630001332435408313313407541252);
    assert_eq!(rand.next_f32(), 0.06663418);
    assert_eq!(rand.next_f64(), 0.9475582258793959);
}