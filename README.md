# heh

Rust implementation of the [xoroshiro128+, xorshift1024*φ,
splitmix64](http://xoroshiro.di.unimi.it), and [Linnorm](https://github.com/tommyettinger/sarong)
random number generators.

Heh is [an Ancient Egyptian deity of primordial chaos](https://en.wikipedia.org/wiki/Heh_(god)),
which makes some sense for a random number generator library.

## License

`heh` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

## Other projects

* Most of this code is directly from [xoroshiro by vks](https://github.com/vks/xoroshiro/);
  only Linnorm and some SIMD tweaks were added, as well as build fixes.
* Parts of the code is were taken from [this pull
  request](https://github.com/rust-lang-nursery/rand/pull/102).
* Some of the test vectors were taken and adapted from the [xorshift crate](
  https://github.com/astocko/xorshift).
* The [xoroshiro128 crate](https://github.com/mscharley/rust-xoroshiro128) is
  similar to this one.
