use criterion::{black_box, criterion_group, criterion_main, Criterion};
use random_numbers;
use random_numbers::RandomNumberAlgorithm;

fn bench_mt_init_with_no_seed(c: &mut Criterion) {
    c.bench_function("init_with_no_seed", |b| b.iter(|| random_numbers::new(RandomNumberAlgorithm::MersenneTwister)));
}

fn bench_mt_init_with_seed(c: &mut Criterion) {
    c.bench_function("init_with_seed", |b| b.iter(|| random_numbers::from_seed(RandomNumberAlgorithm::MersenneTwister, black_box(0))));
}

fn bench_mt_get_random_u64s(c: &mut Criterion) {
    let mut rnd = random_numbers::from_seed(RandomNumberAlgorithm::MersenneTwister, black_box(0));
    
    c.bench_function("get_random_u64s", |b| b.iter(|| rnd.next_u64()));
}

criterion_group!(init_mt_benches, bench_mt_init_with_no_seed, bench_mt_init_with_seed);
criterion_group!(
    name = generate_mt_benches; 
    config = Criterion::default().sample_size(1000);
    targets = bench_mt_get_random_u64s
);

criterion_main!(init_mt_benches, generate_mt_benches);
