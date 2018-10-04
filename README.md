# Small RNGs

[![Build Status](https://travis-ci.org/rust-random/small-rngs.svg?branch=master)](https://travis-ci.org/rust-random/small-rngs)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/rust-random/small-rngs?svg=true)](https://ci.appveyor.com/project/rust-random/small-rngs)
[![Documentation](https://docs.rs/rand/badge.svg)](https://rust-random.github.io/small-rngs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.22+-yellow.svg)](https://github.com/rust-random/rand#rust-version-requirements)
[![License](https://img.shields.io/crates/l/rand.svg)](https://github.com/rust-random/small-rngs#license)

This repository houses a collection of random number generators for use with the
[Rand project](https://github.com/rust-random/rand).

## Sub-crates

All implementations are housed in sub-crates, as follows.

### PCG

Implements a selection of PCG random number generators.

> PCG is a family of simple fast space-efficient statistically good algorithms
> for random number generation. [Melissa O'Neill, Harvey Mudd College, 2014].

- [Link to repository](https://github.com/rust-random/small-rngs/tree/master/rand_pcg)
- [Link to crate](https://crates.io/crates/rand_pcg)

### Xorshift

Implements the Xorshift[^1] random number generator.

[^1]: Marsaglia, George (July 2003).
      ["Xorshift RNGs"](https://www.jstatsoft.org/v08/i14/paper).
      *Journal of Statistical Software*. Vol. 8 (Issue 14).

- [Link to repository](https://github.com/rust-random/small-rngs/tree/master/rand_xorshift)
- [Link to crate](https://crates.io/crates/rand_xorshift)

## Features and dependencies

Wherever possible, all sub-crates are `no_std` compatible, and depend only
`core` and the [`rand_core`](https://crates.io/crates/rand-core) library.

## Tests

All PRNGs feature at minimum a "true values" test comparing output against
test vectors provided as part of the specification, as well as "construction"
tests testing reproducibility of supported seeding methods.

## Benchmarks

This parent crate includes benchmarks of all sub-crates, making benchmarking
as simple as `cargo +nightly bench`.

# License

These crates are distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
