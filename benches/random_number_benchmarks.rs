use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use random_numbers;
use random_numbers::RandomNumber;

fn bench_init_with_no_seed(c: &mut Criterion) {
    c.bench_function("init_with_no_seed", |b| b.iter(|| random_numbers::new_default()));
}

fn bench_init_with_seed(c: &mut Criterion) {
    c.bench_function("init_with_seed", |b| b.iter(|| random_numbers::from_seed_default(black_box(0))));
}

fn bench_get_random_u64s(c: &mut Criterion) {
    let mut rnd = random_numbers::from_seed_default(black_box(0));
    
    c.bench_function("get_random_u64s", |b| b.iter(|| rnd.next_u64()));
}

fn bench_rand_rng_crate(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function("get_random_u64_from_thread_rng", |b| b.iter(|| rng.next_u64()));
}

criterion_group!(init_random_number_benches, bench_init_with_no_seed, bench_init_with_seed);
criterion_group!(
    name = generate_random_number_benches; 
    config = Criterion::default().sample_size(1000);
    targets = bench_get_random_u64s, bench_rand_rng_crate
);

criterion_main!(init_random_number_benches, generate_random_number_benches);
