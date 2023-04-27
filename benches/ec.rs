use criterion::*;
use pasta_curves::group::Group;
use rand_core::OsRng;

use ark_ec::{pairing::Pairing, *};
use ark_std::{ops::Mul, UniformRand, Zero};

use elliptic_curve::ff::Field;
use elliptic_curve::ops::LinearCombination;
use ff::Field as ff13Field;

fn bench_mul(c: &mut Criterion) {
    let mut g = c.benchmark_group("EC::Point::Mul");
    g.sample_size(10);
    g.throughput(Throughput::Elements(1));

    g.bench_function("ark-secp256k1", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G1", |b| {
        let g = ark_bls12_381::G1Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G2", |b| {
        let g = ark_bls12_381::G2Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("k256", |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("p256", |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G1", |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G2", |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("alkali/ed25519", |b| {
        let n = alkali::curve::ed25519::Scalar::generate().unwrap();

        b.iter(|| {
            let r = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();
            let _ = black_box(r);
        })
    });

    g.bench_function("pallas", |b| {
        let n = pasta_curves::pallas::Scalar::random(OsRng);

        b.iter(|| {
            let r = pasta_curves::pallas::Point::generator() * n;
            let _ = black_box(r);
        })
    });

    g.bench_function("vesta", |b| {
        let n = pasta_curves::vesta::Scalar::random(OsRng);

        b.iter(|| {
            let r = pasta_curves::vesta::Point::generator() * n;
            let _ = black_box(r);
        })
    });

    // ---- Mul Projective

    g.bench_function("ark-secp256k1/Projective", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1/Projective", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519/Projective", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G1/Projective", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G2/Projective", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("k256/Projective", |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("p256/Projective", |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G1/Projective", |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G2/Projective", |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    // ---- Mul by zero

    g.bench_function("ark-secp256k1/Zero", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256k1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1/Zero", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256r1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519/Zero", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_curve25519::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G1/Zero", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G2/Zero", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function("k256/Zero", |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * k256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function("p256/Zero", |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * p256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G1/Zero", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G2/Zero", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function("pallas/Zero", |b| {
        let n = pasta_curves::pallas::Scalar::zero();

        b.iter(|| {
            let r = pasta_curves::pallas::Point::generator() * n;
            let _ = black_box(r);
        })
    });

    g.bench_function("vesta/Zero", |b| {
        let n = pasta_curves::vesta::Scalar::zero();

        b.iter(|| {
            let r = pasta_curves::vesta::Point::generator() * n;
            let _ = black_box(r);
        })
    });
}

fn bench_add(c: &mut Criterion) {
    let mut g = c.benchmark_group("EC::Point::Add");
    g.sample_size(10);

    g.bench_function("ark-secp256k1", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G1", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G2", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function("k256", |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let g = k256::AffinePoint::GENERATOR;
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function("p256", |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let g = p256::AffinePoint::GENERATOR;
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G1", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G2", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function("alkali/ed25519", |b| {
        let n = alkali::curve::ed25519::Scalar::generate().unwrap();
        let q = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();

        let n = alkali::curve::ed25519::Scalar::generate().unwrap();
        let p = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();

        alkali::require_init().unwrap();

        b.iter(|| {
            pub const POINT_LENGTH: usize =
                alkali::libsodium_sys::crypto_core_ed25519_BYTES as usize;

            let mut r = [0u8; POINT_LENGTH];
            let add_result = unsafe {
                // SAFETY: Each argument to this function should be the compressed representation of a
                // point on Ed25519, of length `crypto_core_ed25519_BYTES`. We define the `Point` type
                // and `r` array to store this many bytes, so `r` is valid for writes of the required
                // length, and `p`, `q` are valid for reads of the required length.
                alkali::libsodium_sys::crypto_core_ed25519_add(
                    r.as_mut_ptr(),
                    q.0.as_ptr(),
                    p.0.as_ptr(),
                )
            };

            assert_eq!(add_result, 0);

            let _ = black_box(alkali::curve::ed25519::Point(r));
        })
    });

    g.bench_function("pallas", |b| {
        let n = pasta_curves::pallas::Scalar::random(OsRng);
        let p = pasta_curves::pallas::Point::generator() * n;

        let n = pasta_curves::pallas::Scalar::random(OsRng);
        let q = pasta_curves::pallas::Point::generator() * n;

        b.iter(|| {
            let r = p + q;
            let _ = black_box(r);
        })
    });

    g.bench_function("vesta", |b| {
        let n = pasta_curves::vesta::Scalar::random(OsRng);
        let p = pasta_curves::vesta::Point::generator() * n;

        let n = pasta_curves::vesta::Scalar::random(OsRng);
        let q = pasta_curves::vesta::Point::generator() * n;

        b.iter(|| {
            let r = p + q;
            let _ = black_box(r);
        })
    });
}

fn bench_to_affine(c: &mut Criterion) {
    let mut g = c.benchmark_group("EC::Point::ToAffine");
    g.sample_size(10);
    g.throughput(Throughput::Elements(1));

    g.bench_function("ark-secp256k1", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G1", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-bls12-381/G2", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("k256", |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("p256", |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G1", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G1Affine::from(&u);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381/G2", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G2Affine::from(&u);
            let _ = black_box(r);
        })
    });
}

fn bench_pedersen(c: &mut Criterion) {
    let mut g = c.benchmark_group("EC::Point::Pedersen");
    g.sample_size(10);
    g.throughput(Throughput::Elements(1));

    g.bench_function("ark-secp256k1", |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let h = g.mul(s);
        let r = ark_secp256k1::Fr::rand(&mut OsRng);
        let m = ark_secp256k1::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-secp256r1", |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let h = g.mul(s);
        let r = ark_secp256r1::Fr::rand(&mut OsRng);
        let m = ark_secp256r1::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function("ark-curve25519", |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let h = g.mul(s);
        let r = ark_curve25519::Fr::rand(&mut OsRng);
        let m = ark_curve25519::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function("k256", |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let g = k256::ProjectivePoint::GENERATOR;
        let h = g.mul(s);
        let r = k256::Scalar::random(&mut OsRng);
        let m = k256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = k256::ProjectivePoint::lincomb(&h, &m, &g, &r);
            let _ = black_box(r);
        })
    });

    g.bench_function("p256", |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let g = p256::ProjectivePoint::GENERATOR;
        let h = g.mul(s);
        let r = p256::Scalar::random(&mut OsRng);
        let m = p256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = p256::ProjectivePoint::lincomb(&h, &m, &g, &r);
            let _ = black_box(r);
        })
    });

    g.bench_function("pallas", |b| {
        let n = pasta_curves::pallas::Scalar::random(OsRng);
        let q = pasta_curves::pallas::Point::generator() * n;

        let s = pasta_curves::pallas::Scalar::random(OsRng);
        let g = pasta_curves::pallas::Point::generator();
        let h = g * s;
        let r = pasta_curves::pallas::Scalar::random(OsRng);
        let m = pasta_curves::pallas::Scalar::random(OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function("vesta", |b| {
        let n = pasta_curves::vesta::Scalar::random(OsRng);
        let q = pasta_curves::vesta::Point::generator() * n;

        let s = pasta_curves::vesta::Scalar::random(OsRng);
        let g = pasta_curves::vesta::Point::generator();
        let h = g * s;
        let r = pasta_curves::vesta::Scalar::random(OsRng);
        let m = pasta_curves::vesta::Scalar::random(OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });
}

fn bench_bls_pairing(c: &mut Criterion) {
    let mut g = c.benchmark_group("EC::Point::Pairing");
    g.sample_size(10);
    g.throughput(Throughput::Elements(1));

    g.bench_function("ark-bls12-381", |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = ark_bls12_381::G2Affine::generator().mul(&s).into_affine();

        b.iter(|| {
            let r = ark_bls12_381::Bls12_381::pairing(g, u);
            let _ = black_box(r);
        })
    });

    g.bench_function("bls12-381", |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = bls12_381::G2Affine::from(&bls12_381::G2Affine::generator().mul(s));

        b.iter(|| {
            let r = bls12_381::pairing(&g, &u);
            let _ = black_box(r);
        })
    });

    g.bench_function("blst", |b| {
        let g = unsafe { blst::BLS12_381_G1 };
        let u = unsafe { blst::BLS12_381_G2 };

        b.iter(|| {
            let mut pairing = blst::Pairing::new(false, &[]);
            pairing.raw_aggregate(&u, &g);
            let r = pairing.as_fp12();
            let _ = black_box(r);
        })
    });
}

criterion_group!(
    benches,
    bench_mul,
    bench_add,
    bench_to_affine,
    bench_pedersen
);
criterion_main!(benches);
