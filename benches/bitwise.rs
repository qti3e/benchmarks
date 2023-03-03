use benchmarks::*;
use criterion::*;

fn bench_bitwise(c: &mut Criterion) {
    let mut g = c.benchmark_group("Bitwise");
    g.sample_size(20);
    g.throughput(Throughput::Bytes(GB as u64));

    let v1 = random_vec(GB);
    let v2 = random_vec(GB);

    g.bench_function("xor", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            for i in 0..GB {
                result[i] = v1[i] ^ v2[i];
            }
            black_box(&result);
        })
    });

    g.bench_function("and", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            for i in 0..GB {
                result[i] = v1[i] & v2[i];
            }
            black_box(&result);
        })
    });

    g.bench_function("or", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            for i in 0..GB {
                result[i] = v1[i] | v2[i];
            }
            black_box(&result);
        })
    });

    g.finish();
}

criterion_group!(benches, bench_bitwise);
criterion_main!(benches);
