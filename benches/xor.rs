use benchmarks::*;
use criterion::*;

/// This benchmark tries to xor two 1GB vector and store the result into a 3rd vector.
fn bench_xor(c: &mut Criterion) {
    let mut g = c.benchmark_group("XOR");
    g.sample_size(20);
    g.throughput(Throughput::Bytes(GB as u64));

    let v1 = random_vec(GB);
    let v2 = random_vec(GB);

    g.bench_function("naive", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            for i in 0..GB {
                result[i] = v1[i] ^ v2[i];
            }
            black_box(&result);
        })
    });

    g.bench_function("unchecked", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            for i in 0..GB {
                unsafe {
                    *result.get_unchecked_mut(i) = v1.get_unchecked(i) ^ v2.get_unchecked(i);
                }
            }
            black_box(&result);
        })
    });

    g.bench_function("u64", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            unsafe {
                let result_as_u64_slice: &mut Vec<u64> = std::mem::transmute(&mut result);
                let v1_as_u64_slice: &Vec<u64> = std::mem::transmute(&v1);
                let v2_as_u64_slice: &Vec<u64> = std::mem::transmute(&v2);
                for i in 0..GB / 8 {
                    *result_as_u64_slice.get_unchecked_mut(i) =
                        v1_as_u64_slice.get_unchecked(i) ^ v2_as_u64_slice.get_unchecked(i);
                }
            }
            black_box(&result);
        })
    });

    g.bench_function("packed_simd::u64x8", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| {
            unsafe {
                let result_as_u64_slice: &mut Vec<u64> = std::mem::transmute(&mut result);
                let v1_as_u64_slice: &Vec<u64> = std::mem::transmute(&v1);
                let v2_as_u64_slice: &Vec<u64> = std::mem::transmute(&v2);
                let mut offset = 0;

                loop {
                    if offset >= GB / 8 {
                        break;
                    }

                    let v1_simd = packed_simd::u64x8::new(
                        *v1_as_u64_slice.get_unchecked(offset),
                        *v1_as_u64_slice.get_unchecked(offset + 1),
                        *v1_as_u64_slice.get_unchecked(offset + 2),
                        *v1_as_u64_slice.get_unchecked(offset + 3),
                        *v1_as_u64_slice.get_unchecked(offset + 4),
                        *v1_as_u64_slice.get_unchecked(offset + 5),
                        *v1_as_u64_slice.get_unchecked(offset + 6),
                        *v1_as_u64_slice.get_unchecked(offset + 7),
                    );

                    let v2_simd = packed_simd::u64x8::new(
                        *v2_as_u64_slice.get_unchecked(offset),
                        *v2_as_u64_slice.get_unchecked(offset + 1),
                        *v2_as_u64_slice.get_unchecked(offset + 2),
                        *v2_as_u64_slice.get_unchecked(offset + 3),
                        *v2_as_u64_slice.get_unchecked(offset + 4),
                        *v2_as_u64_slice.get_unchecked(offset + 5),
                        *v2_as_u64_slice.get_unchecked(offset + 6),
                        *v2_as_u64_slice.get_unchecked(offset + 7),
                    );

                    let r_simd = v1_simd ^ v2_simd;
                    r_simd.write_to_slice_aligned_unchecked(&mut result_as_u64_slice[offset..]);

                    offset += 8;
                }
            }

            black_box(&result);
        })
    });

    g.bench_function("packed_simd::u64x4", |b| {
        let mut result = mk_vec(GB);
        b.iter(|| unsafe {
            let result_as_u64_slice: &mut Vec<u64> = std::mem::transmute(&mut result);
            let v1_as_u64_slice: &Vec<u64> = std::mem::transmute(&v1);
            let v2_as_u64_slice: &Vec<u64> = std::mem::transmute(&v2);
            let mut offset = 0;

            loop {
                if offset >= GB / 8 {
                    break;
                }

                let v1_simd = packed_simd::u64x4::new(
                    *v1_as_u64_slice.get_unchecked(offset),
                    *v1_as_u64_slice.get_unchecked(offset + 1),
                    *v1_as_u64_slice.get_unchecked(offset + 2),
                    *v1_as_u64_slice.get_unchecked(offset + 3),
                );

                let v2_simd = packed_simd::u64x4::new(
                    *v2_as_u64_slice.get_unchecked(offset),
                    *v2_as_u64_slice.get_unchecked(offset + 1),
                    *v2_as_u64_slice.get_unchecked(offset + 2),
                    *v2_as_u64_slice.get_unchecked(offset + 3),
                );

                let r_simd = v1_simd ^ v2_simd;
                r_simd.write_to_slice_aligned_unchecked(&mut result_as_u64_slice[offset..]);

                offset += 4;
            }

            black_box(&result);
        })
    });

    g.bench_function("packed_simd::u8x64", |b| {
        let mut result = mk_vec(GB);

        b.iter(|| unsafe {
            let mut offset = 0;
            loop {
                if offset >= GB {
                    break;
                }

                let v1_simd = packed_simd::u8x64::new(
                    *v1.get_unchecked(offset),
                    *v1.get_unchecked(offset + 1),
                    *v1.get_unchecked(offset + 2),
                    *v1.get_unchecked(offset + 3),
                    *v1.get_unchecked(offset + 4),
                    *v1.get_unchecked(offset + 5),
                    *v1.get_unchecked(offset + 6),
                    *v1.get_unchecked(offset + 7),
                    *v1.get_unchecked(offset + 8),
                    *v1.get_unchecked(offset + 9),
                    *v1.get_unchecked(offset + 10),
                    *v1.get_unchecked(offset + 11),
                    *v1.get_unchecked(offset + 12),
                    *v1.get_unchecked(offset + 13),
                    *v1.get_unchecked(offset + 14),
                    *v1.get_unchecked(offset + 15),
                    *v1.get_unchecked(offset + 16),
                    *v1.get_unchecked(offset + 17),
                    *v1.get_unchecked(offset + 18),
                    *v1.get_unchecked(offset + 19),
                    *v1.get_unchecked(offset + 20),
                    *v1.get_unchecked(offset + 21),
                    *v1.get_unchecked(offset + 22),
                    *v1.get_unchecked(offset + 23),
                    *v1.get_unchecked(offset + 24),
                    *v1.get_unchecked(offset + 25),
                    *v1.get_unchecked(offset + 26),
                    *v1.get_unchecked(offset + 27),
                    *v1.get_unchecked(offset + 28),
                    *v1.get_unchecked(offset + 29),
                    *v1.get_unchecked(offset + 30),
                    *v1.get_unchecked(offset + 31),
                    *v1.get_unchecked(offset + 32),
                    *v1.get_unchecked(offset + 33),
                    *v1.get_unchecked(offset + 34),
                    *v1.get_unchecked(offset + 35),
                    *v1.get_unchecked(offset + 36),
                    *v1.get_unchecked(offset + 37),
                    *v1.get_unchecked(offset + 38),
                    *v1.get_unchecked(offset + 39),
                    *v1.get_unchecked(offset + 40),
                    *v1.get_unchecked(offset + 41),
                    *v1.get_unchecked(offset + 42),
                    *v1.get_unchecked(offset + 43),
                    *v1.get_unchecked(offset + 44),
                    *v1.get_unchecked(offset + 45),
                    *v1.get_unchecked(offset + 46),
                    *v1.get_unchecked(offset + 47),
                    *v1.get_unchecked(offset + 48),
                    *v1.get_unchecked(offset + 49),
                    *v1.get_unchecked(offset + 50),
                    *v1.get_unchecked(offset + 51),
                    *v1.get_unchecked(offset + 52),
                    *v1.get_unchecked(offset + 53),
                    *v1.get_unchecked(offset + 54),
                    *v1.get_unchecked(offset + 55),
                    *v1.get_unchecked(offset + 56),
                    *v1.get_unchecked(offset + 57),
                    *v1.get_unchecked(offset + 58),
                    *v1.get_unchecked(offset + 59),
                    *v1.get_unchecked(offset + 60),
                    *v1.get_unchecked(offset + 61),
                    *v1.get_unchecked(offset + 62),
                    *v1.get_unchecked(offset + 63),
                );

                let v2_simd = packed_simd::u8x64::new(
                    *v2.get_unchecked(offset),
                    *v2.get_unchecked(offset + 1),
                    *v2.get_unchecked(offset + 2),
                    *v2.get_unchecked(offset + 3),
                    *v2.get_unchecked(offset + 4),
                    *v2.get_unchecked(offset + 5),
                    *v2.get_unchecked(offset + 6),
                    *v2.get_unchecked(offset + 7),
                    *v2.get_unchecked(offset + 8),
                    *v2.get_unchecked(offset + 9),
                    *v2.get_unchecked(offset + 10),
                    *v2.get_unchecked(offset + 11),
                    *v2.get_unchecked(offset + 12),
                    *v2.get_unchecked(offset + 13),
                    *v2.get_unchecked(offset + 14),
                    *v2.get_unchecked(offset + 15),
                    *v2.get_unchecked(offset + 16),
                    *v2.get_unchecked(offset + 17),
                    *v2.get_unchecked(offset + 18),
                    *v2.get_unchecked(offset + 19),
                    *v2.get_unchecked(offset + 20),
                    *v2.get_unchecked(offset + 21),
                    *v2.get_unchecked(offset + 22),
                    *v2.get_unchecked(offset + 23),
                    *v2.get_unchecked(offset + 24),
                    *v2.get_unchecked(offset + 25),
                    *v2.get_unchecked(offset + 26),
                    *v2.get_unchecked(offset + 27),
                    *v2.get_unchecked(offset + 28),
                    *v2.get_unchecked(offset + 29),
                    *v2.get_unchecked(offset + 30),
                    *v2.get_unchecked(offset + 31),
                    *v2.get_unchecked(offset + 32),
                    *v2.get_unchecked(offset + 33),
                    *v2.get_unchecked(offset + 34),
                    *v2.get_unchecked(offset + 35),
                    *v2.get_unchecked(offset + 36),
                    *v2.get_unchecked(offset + 37),
                    *v2.get_unchecked(offset + 38),
                    *v2.get_unchecked(offset + 39),
                    *v2.get_unchecked(offset + 40),
                    *v2.get_unchecked(offset + 41),
                    *v2.get_unchecked(offset + 42),
                    *v2.get_unchecked(offset + 43),
                    *v2.get_unchecked(offset + 44),
                    *v2.get_unchecked(offset + 45),
                    *v2.get_unchecked(offset + 46),
                    *v2.get_unchecked(offset + 47),
                    *v2.get_unchecked(offset + 48),
                    *v2.get_unchecked(offset + 49),
                    *v2.get_unchecked(offset + 50),
                    *v2.get_unchecked(offset + 51),
                    *v2.get_unchecked(offset + 52),
                    *v2.get_unchecked(offset + 53),
                    *v2.get_unchecked(offset + 54),
                    *v2.get_unchecked(offset + 55),
                    *v2.get_unchecked(offset + 56),
                    *v2.get_unchecked(offset + 57),
                    *v2.get_unchecked(offset + 58),
                    *v2.get_unchecked(offset + 59),
                    *v2.get_unchecked(offset + 60),
                    *v2.get_unchecked(offset + 61),
                    *v2.get_unchecked(offset + 62),
                    *v2.get_unchecked(offset + 63),
                );

                let r_simd = v1_simd ^ v2_simd;
                r_simd.write_to_slice_aligned_unchecked(&mut result[offset..]);

                offset += 64;
            }

            black_box(&result);
        })
    });

    g.finish();
}

criterion_group!(benches, bench_xor);
criterion_main!(benches);