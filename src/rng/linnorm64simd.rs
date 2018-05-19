use rand_core::{Error, RngCore, SeedableRng};
use rand_core::block::{BlockRngCore, BlockRng};
use faster::Transmute;
use faster::vecs::{u64x4};
use byteorder::{LittleEndian, ByteOrder};

use super::SplitMix64;

/// A linnorm64 random number generator using SIMD to generate 4 `u64` at a time.
///
/// The Linnorm algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XoroShiro128`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Linnorm64x4Core {
    x: u64x4,
}

impl Linnorm64x4Core {
    /// Return the next random `u64x4`.
    #[inline]
    pub fn next_u64x4(&mut self) -> u64x4 {
        const LCG_MUL : u64x4 = u64x4::new(0x41C64E6Du64, 0x41C64E6Du64, 0x41C64E6Du64, 0x41C64E6Du64);
        const LCG_INC : u64x4 = u64x4::new(0xBE9u64, 0xACEDu64, 0xDE4Du64, 0x1u64);
        const Z_MUL : u64x4 = u64x4::new(0xAEF17502108EF2D9u64, 0xAEF17502108EF2D9u64, 0xAEF17502108EF2D9u64, 0xAEF17502108EF2D9u64);
        self.x = self.x * LCG_MUL + LCG_INC;
        let z = (self.x ^ (self.x >> 32)) * Z_MUL;
        z ^ (z >> 30)
    }

    /// Create a new `Linnorm64x4Core`.  This will use `SplitMix64` to fill the seed.
    #[inline]
    pub fn from_seed_u64(seed: u64) -> Linnorm64x4Core {
        let mut rng = SplitMix64::from_seed_u64(seed);
        Linnorm64x4Core::from_seed(Linnorm64x4Seed::from_rng(&mut rng))
    }
}

pub struct Linnorm64x4Seed([u8; 32]);

/// Seed for a `Linnorm64x4` or `Linnorm64x4Core`.
impl Linnorm64x4Seed {
    #[inline]
    /// Create a seed for a `Linnorm64x4` or `Linnorm64x4Core`.
    pub fn new(seed: [u8; 32]) -> Linnorm64x4Seed {
        Linnorm64x4Seed(seed)
    }

    /// Use an RNG to generate a valid linnorm seed.
    pub fn from_rng<R: RngCore>(rng: &mut R) -> Linnorm64x4Seed {
        let mut seed = [0; 32];
        rng.fill_bytes(&mut seed);
        Linnorm64x4Seed(seed)
    }
}

impl ::std::convert::AsMut<[u8]> for Linnorm64x4Seed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl ::std::default::Default for Linnorm64x4Seed {
    fn default() -> Linnorm64x4Seed {
        Linnorm64x4Seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31])
    }
}

impl SeedableRng for Linnorm64x4Core {
    type Seed = Linnorm64x4Seed;

    /// Create a new `Linnorm64x4Core`.
    #[inline]
    fn from_seed(seed: Linnorm64x4Seed) -> Linnorm64x4Core {
        let seed = seed.0;
        Linnorm64x4Core {
            x: u64x4::new(
                    LittleEndian::read_u64(&seed[0..8]),
                    LittleEndian::read_u64(&seed[8..16]),
                    LittleEndian::read_u64(&seed[16..24]),
                    LittleEndian::read_u64(&seed[24..32]),
                ),
        }
    }
}

impl BlockRngCore for Linnorm64x4Core {
    type Item = u32;
    type Results = [u32; 8];

    #[inline]
    fn generate(&mut self, results: &mut Self::Results) {
        let r = self.next_u64x4().be_u32s();
        r.store(results, 0);
    }
}

#[derive(Clone, Debug)]
pub struct Linnorm64x4(BlockRng<Linnorm64x4Core>);

impl Linnorm64x4 {
    /// Create a new `Linnorm64x4`.  This will use `SplitMix64` to fill the seed.
    #[inline]
    pub fn from_seed_u64(seed: u64) -> Self {
        Linnorm64x4(BlockRng::<Linnorm64x4Core>::new(Linnorm64x4Core::from_seed_u64(seed)))
    }
}

impl RngCore for Linnorm64x4 {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl SeedableRng for Linnorm64x4 {
    type Seed = <Linnorm64x4Core as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Linnorm64x4(BlockRng::<Linnorm64x4Core>::from_seed(seed))
    }

    fn from_rng<R: RngCore>(rng: R) -> Result<Self, Error> {
        BlockRng::<Linnorm64x4Core>::from_rng(rng).map(|rng| Linnorm64x4(rng))
    }
}

#[test]
fn test_vs_non_simd() {
    use ::rand_core::SeedableRng;
    use super::Linnorm64;

    let mut seed = [0; 32];
    LittleEndian::write_u64(&mut seed[0..8], 0);
    LittleEndian::write_u64(&mut seed[8..16], 1);
    LittleEndian::write_u64(&mut seed[16..24], 2);
    LittleEndian::write_u64(&mut seed[24..32], 3);

    let mut rng_simd = Linnorm64x4Core::from_seed(
        Linnorm64x4Seed::new(seed));

    fn linnorm_from_slice(slice: &[u8]) -> Linnorm64 {
        let mut seed = [0; 8];
        for (x, y) in slice.iter().zip(seed.iter_mut()) {
            *y = *x;
        }
        Linnorm64::from_seed(seed)
    }

    let mut rngs = [
        linnorm_from_slice(&seed[0..8]),
        linnorm_from_slice(&seed[8..16]),
        linnorm_from_slice(&seed[16..24]),
        linnorm_from_slice(&seed[24..32]),
    ];

    let r_simd = rng_simd.next_u64x4();
    let rs = [
        rngs[0].next_u64(),
        rngs[1].next_u64(),
        rngs[2].next_u64(),
        rngs[3].next_u64(),
    ];
    assert_eq!(r_simd.extract(0), rs[0]);
    assert_eq!(r_simd.extract(1), rs[1]);
    assert_eq!(r_simd.extract(2), rs[2]);
    assert_eq!(r_simd.extract(3), rs[3]);
}
