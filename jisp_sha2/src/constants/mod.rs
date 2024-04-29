//! A collection of constant words for each of the different SHA-2 variations
//! To use the SHA-2 algorithm with different constants you can implement the [Constants] trait

/// Implement this for your constants to be used in the SHA-2 algorithm
/// Note that the SHA-256 algorithm uses a list of 64 constant `u32` words.
/// The SHA-512 algorithm uses a list of 80 constant `u64` words. 
pub trait Constants<const KLEN:usize, T> {
    fn constant_words() -> [T;KLEN];
    fn initial_hash() -> [T;8];
}

pub use sha256_constants::Sha256Constants as Sha256;
pub use sha256_constants::Sha224Constants as Sha224;

pub use sha512_constants::Sha512Constants as Sha512;
pub use sha512_constants::Sha384Constants as Sha384;

mod sha256_constants;
mod sha512_constants;