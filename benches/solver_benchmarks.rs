use criterion::{black_box, criterion_group, criterion_main, Criterion};
use depegsentinel_solana::curve::StableswapSolver;

fn benchmark_stableswap_solver(c: &mut Criterion) {
    let balances = vec![15_000_000.0, 14_980_000.0, 15_020_000.0];
    let a = 120.0;

    c.bench_function("stableswap_compute_d", |b| {
        b.iter(|| {
            StableswapSolver::compute_d(black_box(&balances), black_box(a))
        })
    });

    c.bench_function("stableswap_simulate_swap", |b| {
        b.iter(|| {
            StableswapSolver::simulate_swap(black_box(&balances), black_box(a), 0, 1, black_box(500_000.0)).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_stableswap_solver);
criterion_main!(benches);
