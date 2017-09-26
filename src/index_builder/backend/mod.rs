//! Definition of backends to create character bitmaps

#[cfg(feature = "simd-accel")]
mod sse2;
#[cfg(all(feature = "avx-accel", target_arch = "x86_64"))]
mod avx;
mod fallback;

pub use self::fallback::FallbackBackend;
#[cfg(feature = "simd-accel")]
pub use self::sse2::Sse2Backend;
#[cfg(all(feature = "avx-accel", target_arch = "x86_64"))]
pub use self::avx::AvxBackend;
use super::Bitmap;


/// Represents the backend of `IndexBuilder` to create character bitmaps
pub trait Backend {
    /// Create a new bitmap from slice of bytes
    fn create_full_bitmap(&self, s: &[u8], offset: usize) -> Bitmap;

    /// Create a new bitmap from slice of bytes, whose length may be less than 64.
    fn create_partial_bitmap(&self, s: &[u8], offset: usize) -> Bitmap;
}
