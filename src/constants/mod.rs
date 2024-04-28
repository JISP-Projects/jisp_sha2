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