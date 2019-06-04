use crc32fast::Hasher;
use digest::{impl_write, FixedOutput, Input, Reset};
use generic_array::typenum::U4;
use generic_array::GenericArray;

pub use digest::Digest;

#[derive(Clone, Default)]
/// Wraps a [`Hasher`] and provides it with [`Digest`] and [`DynDigest`] implementations.
///
/// [`Digest`]: ../digest/trait.Digest.html
/// [`DynDigest`]: ../digest/trait.DynDigest.html
/// [`Hasher`]: ../crc32fast/struct.Hasher.html
pub struct Crc32(Hasher);

impl Crc32 {
    /// Creates a new `Crc32`.
    #[inline]
    pub fn new() -> Self {
        Self(Hasher::new())
    }

    /// Creates a new `Crc32` initialized with the given state.
    #[inline]
    pub fn from_state(state: u32) -> Self {
        Self(Hasher::new_with_initial(state))
    }
}

impl FixedOutput for Crc32 {
    type OutputSize = U4;

    #[inline]
    fn fixed_result(self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.0.finalize();
        GenericArray::clone_from_slice(&result.to_be_bytes())
    }
}

impl Input for Crc32 {
    #[inline]
    fn input<B: AsRef<[u8]>>(&mut self, data: B) {
        self.0.update(data.as_ref());
    }
}

impl Reset for Crc32 {
    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}

impl_write!(Crc32);
