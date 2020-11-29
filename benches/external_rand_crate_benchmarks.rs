use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn bench_rand_rng_crate(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function("get_u32_from_rand_crate", |b| b.iter(|| rng.next_u32()));
    c.bench_function("get_u64_from_rand_crate", |b| b.iter(|| rng.next_u64()));
    c.bench_function("get_f32_from_rand_crate", |b| b.iter(|| rng.gen::<f32>()));
    c.bench_function("get_f64_from_rand_crate", |b| b.iter(|| rng.gen::<f64>()));
}

criterion_group!(generate_rand_number_benches, bench_rand_rng_crate);

criterion_main!(generate_rand_number_benches);
