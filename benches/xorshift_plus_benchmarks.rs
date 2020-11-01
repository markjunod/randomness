use criterion::{black_box, criterion_group, criterion_main, Criterion};
use random_numbers;
use random_numbers::RandomNumberAlgorithm;

fn bench_xorshift_plus_init_with_no_seed(c: &mut Criterion) {
    c.bench_function("init_with_no_seed", |b| b.iter(|| random_numbers::new(RandomNumberAlgorithm::XorshiftPlus)));
}

fn bench_xorshift_plus_init_with_seed(c: &mut Criterion) {
    c.bench_function("init_with_seed", |b| b.iter(|| random_numbers::from_seed(RandomNumberAlgorithm::XorshiftPlus, black_box(0))));
}

fn bench_xorshift_plus_get_random_u64s(c: &mut Criterion) {
    let mut rnd = random_numbers::from_seed(RandomNumberAlgorithm::XorshiftPlus, black_box(0));
    
    c.bench_function("get_random_u64s", |b| b.iter(|| rnd.next_u64()));
}

criterion_group!(init_xorhift_plus_benches, bench_xorshift_plus_init_with_no_seed, bench_xorshift_plus_init_with_seed);
criterion_group!(
    name = generate_xorhift_plus_benches; 
    config = Criterion::default().sample_size(1000);
    targets = bench_xorshift_plus_get_random_u64s
);

criterion_main!(init_xorhift_plus_benches, generate_xorhift_plus_benches);