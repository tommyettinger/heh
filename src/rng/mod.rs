mod splitmix64;
mod linnorm64;
mod xoroshiro128;
#[cfg(feature = "unstable")]
mod xoroshiro128simd;
#[cfg(feature = "unstable")]
mod linnorm64simd;
#[cfg(feature = "unstable")]
mod splitmix64simd;
mod xorshift1024;

pub use self::splitmix64::SplitMix64;
pub use self::linnorm64::Linnorm64;
pub use self::xoroshiro128::XoroShiro128;
#[cfg(feature = "unstable")]
pub use self::xoroshiro128simd::{XoroShiro128x4, XoroShiro128x4Seed};
#[cfg(feature = "unstable")]
pub use self::linnorm64simd::{Linnorm64x4, Linnorm64x4Seed};
#[cfg(feature = "unstable")]
pub use self::splitmix64simd::{SplitMix64x4, SplitMix64x4Seed};
pub use self::xorshift1024::{XorShift1024, XorShift1024Seed};
