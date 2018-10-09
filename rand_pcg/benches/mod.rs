#![feature(test)]

extern crate test;
extern crate rand_core;
extern crate rand_pcg;

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;
const SEED: u64 = 1234567890;

use std::mem::size_of_val;
use test::{black_box, Bencher};

use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Mcg128Xsl64};

#[inline(never)]
fn get_seed() -> u64 {
    SEED
}

macro_rules! gen_bytes {
    ($fnn:ident, $gen:ty) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = <$gen>::seed_from_u64(get_seed());
            let mut buf = [0u8; BYTES_LEN];
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    rng.fill_bytes(&mut buf);
                    black_box(buf);
                }
            });
            b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
        }
    }
}

gen_bytes!(gen_bytes_lcg64_xsh32, Lcg64Xsh32);
gen_bytes!(gen_bytes_mcg128_xsh64, Mcg128Xsl64);

macro_rules! gen_uint {
    ($fnn:ident, $ty:expr, $gen:ty) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = <$gen>::seed_from_u64(get_seed());
            let mut accum = $ty(&mut rng);
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    accum = accum.wrapping_add($ty(&mut rng));
                }
                accum
            });
            b.bytes = size_of_val(&accum) as u64 * RAND_BENCH_N;
        }
    }
}

gen_uint!(gen_u32_lcg64_xsh32, RngCore::next_u32, Lcg64Xsh32);
gen_uint!(gen_u32_mcg128_xsh64, RngCore::next_u32, Mcg128Xsl64);
gen_uint!(gen_u64_lcg64_xsh32, RngCore::next_u64, Lcg64Xsh32);
gen_uint!(gen_u64_mcg128_xsh64, RngCore::next_u64, Mcg128Xsl64);

macro_rules! init_gen {
    ($fnn:ident, $gen:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            b.iter(|| {
                let r2 = <$gen>::seed_from_u64(get_seed());
                r2
            });
        }
    }
}

init_gen!(init_lcg64_xsh32, Lcg64Xsh32);
init_gen!(init_mcg128_xsh64, Mcg128Xsl64);
