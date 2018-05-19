use rand_core;
use rand_core::{RngCore, SeedableRng};
use byteorder::{LittleEndian, ByteOrder};

/// A Linnorm random number generator.
///
/// The Linnorm algorithm is not suitable for cryptographic purposes, but is
/// very fast, has high quality, and has a 64 bit state. It does not fail the
/// statistical tests `XoroShiro128` fails systematically.
/// If you do not know for sure that it fits your requirements, use a more
/// secure one such as `IsaacRng` or `OsRng`.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Linnorm64 {
    x: u64,
}

impl Linnorm64 {
    /// Creates a new `Linnorm64` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> Linnorm64 {
        // The state can be seeded with any value.
        Linnorm64 {
            x: 0,
        }
    }

    pub fn from_seed_u64(seed: u64) -> Linnorm64 {
        let mut x = [0; 8];
        LittleEndian::write_u64(&mut x, seed);
        Linnorm64::from_seed(x)
    }
}

impl RngCore for Linnorm64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.x = self.x.wrapping_mul(0x41C64E6D).wrapping_add(1);
        let z = (self.x ^ (self.x >> 32)).wrapping_mul(0xAEF17502108EF2D9);
        return z ^ (z >> 30);
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for mut chunk in dest.chunks_mut(8) {
            if chunk.len() == 8 {
                LittleEndian::write_u64(&mut chunk, self.next_u64());
            } else {
                debug_assert!(chunk.len() < 8);
                let r = self.next_u64();
                let mut i = 0;
                for v in chunk.iter_mut() {
                    *v = (r >> 8*i) as u8;
                    i += 1;
                }
            }
        }
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Linnorm64 {
    type Seed = [u8; 8];

    /// Create a new `Linnorm64`.
    fn from_seed(seed: [u8; 8]) -> Linnorm64 {
        Linnorm64 {
            x: LittleEndian::read_u64(&seed),
        }
    }
}
