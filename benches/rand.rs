use benchmarks::*;
use criterion::*;
use rand::{RngCore, SeedableRng};
use std::arch::aarch64 as arch;

fn bench_rand(c: &mut Criterion) {
    let mut g = c.benchmark_group("Rand");
    g.sample_size(20);

    for (size, label) in size(GB) {
        g.throughput(Throughput::Bytes(size as u64));

        g.bench_with_input(BenchmarkId::new("OsRng", &label), &size, |b, &size| {
            let mut vec = mk_vec(size);
            b.iter(|| rand_core::OsRng.fill_bytes(vec.as_mut_slice()));
            black_box(vec);
        });

        g.bench_with_input(BenchmarkId::new("ThreadRng", &label), &size, |b, &size| {
            let mut vec = mk_vec(size);
            let mut rng = rand::thread_rng();
            b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
            black_box(vec);
        });

        g.bench_with_input(
            BenchmarkId::new("rand_chacha/8", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_chacha::ChaCha8Rng::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_chacha/12", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_chacha::ChaCha12Rng::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_chacha/20", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_chacha::ChaCha20Rng::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(BenchmarkId::new("fastrand", &label), &size, |b, &size| {
            let mut vec = mk_vec(size);
            let rng = fastrand::Rng::new();
            b.iter(|| rng.fill(vec.as_mut_slice()));
            black_box(vec);
        });

        g.bench_with_input(BenchmarkId::new("rand_hc", &label), &size, |b, &size| {
            let mut vec = mk_vec(size);
            let mut rng = rand_hc::Hc128Rng::from_seed([27; 32]);
            b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
            black_box(vec);
        });

        g.bench_with_input(
            BenchmarkId::new("rand_xorshift", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_xorshift::XorShiftRng::from_seed([27; 16]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/128+", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_xoshiro::Xoshiro128Plus::from_seed([27; 16]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/256+", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_xoshiro::Xoshiro256Plus::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/512+", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng =
                    rand_xoshiro::Xoshiro512Plus::from_seed(rand_xoshiro::Seed512([27; 64]));
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/128++", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_xoshiro::Xoshiro128PlusPlus::from_seed([27; 16]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/256++", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(
            BenchmarkId::new("rand_xoshiro/512++", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng =
                    rand_xoshiro::Xoshiro512PlusPlus::from_seed(rand_xoshiro::Seed512([27; 64]));
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        g.bench_with_input(BenchmarkId::new("rand_isaac", &label), &size, |b, &size| {
            let mut vec = mk_vec(size);
            let mut rng = rand_isaac::IsaacRng::from_seed([27; 32]);
            b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
            black_box(vec);
        });

        g.bench_with_input(
            BenchmarkId::new("rand_isaac/IssacRng64", &label),
            &size,
            |b, &size| {
                let mut vec = mk_vec(size);
                let mut rng = rand_isaac::Isaac64Rng::from_seed([27; 32]);
                b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
                black_box(vec);
            },
        );

        // rand_jitter is too slow to be worth measuring :|
        // throughput ~ 24Kb/s
        // g.bench_with_input(
        //     BenchmarkId::new("rand_jitter", &label),
        //     &size,
        //     |b, &size| {
        //         let mut vec = mk_vec(size);
        //         let mut rng = rand_jitter::JitterRng::new_with_timer(jitter_nstime);
        //         b.iter(|| rng.fill_bytes(vec.as_mut_slice()));
        //         black_box(vec);
        //     },
        // );
    }

    g.finish();
}

// fn jitter_nstime() -> u64 {
//     use std::time::{SystemTime, UNIX_EPOCH};
//
//     let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     // The correct way to calculate the current time is
//     // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
//     // But this is faster, and the difference in terms of entropy is
//     // negligible (log2(10^9) == 29.9).
//     dur.as_secs() << 30 | dur.subsec_nanos() as u64
// }

criterion_group!(benches, bench_rand);
criterion_main!(benches);
