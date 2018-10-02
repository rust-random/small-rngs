// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate test;
extern crate rand;
extern crate rand_pcg;
extern crate rand_xorshift;

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;

use std::mem::size_of;
use test::{black_box, Bencher};

use rand::prelude::*;
use rand::rngs::{SmallRng, StdRng};
use rand_pcg::{Lcg64Xsh32, Mcg128Xsl64};
use rand_xorshift::XorShiftRng;

macro_rules! gen_bytes {
    ($fnn:ident, $gen:expr) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = $gen;
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

gen_bytes!(gen_bytes_xorshift, XorShiftRng::from_entropy());
gen_bytes!(gen_bytes_lcg64_xsh32, Lcg64Xsh32::from_entropy());
gen_bytes!(gen_bytes_mcg128_xsh64, Mcg128Xsl64::from_entropy());
gen_bytes!(gen_bytes_std, StdRng::from_entropy());
gen_bytes!(gen_bytes_small, SmallRng::from_entropy());

macro_rules! gen_uint {
    ($fnn:ident, $ty:ty, $gen:expr) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = $gen;
            b.iter(|| {
                let mut accum: $ty = 0;
                for _ in 0..RAND_BENCH_N {
                    accum = accum.wrapping_add(rng.gen::<$ty>());
                }
                accum
            });
            b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
        }
    }
}

gen_uint!(gen_u32_xorshift, u32, XorShiftRng::from_entropy());
gen_uint!(gen_u32_lcg64_xsh32, u32, Lcg64Xsh32::from_entropy());
gen_uint!(gen_u32_mcg128_xsh64, u32, Mcg128Xsl64::from_entropy());
gen_uint!(gen_u32_std, u32, StdRng::from_entropy());
gen_uint!(gen_u32_small, u32, SmallRng::from_entropy());

gen_uint!(gen_u64_xorshift, u64, XorShiftRng::from_entropy());
gen_uint!(gen_u64_lcg64_xsh32, u64, Lcg64Xsh32::from_entropy());
gen_uint!(gen_u64_mcg128_xsh64, u64, Mcg128Xsl64::from_entropy());
gen_uint!(gen_u64_std, u64, StdRng::from_entropy());
gen_uint!(gen_u64_small, u64, SmallRng::from_entropy());

macro_rules! init_gen {
    ($fnn:ident, $gen:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = XorShiftRng::from_entropy();
            b.iter(|| {
                let r2 = $gen::from_rng(&mut rng).unwrap();
                r2
            });
        }
    }
}

init_gen!(init_xorshift, XorShiftRng);
init_gen!(init_lcg64_xsh32, Lcg64Xsh32);
init_gen!(init_mcg128_xsh64, Mcg128Xsl64);
init_gen!(init_std, StdRng);
init_gen!(init_small, SmallRng);
