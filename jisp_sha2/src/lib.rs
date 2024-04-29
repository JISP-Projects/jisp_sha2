//! # About
//! This crate contains my implementation of SHA256, SHA512 and their variants SHA224 and SHA384. 
//!
//! # Security
//! This implementation is just my personal project and has not been officially verified or audited.
//! It should therefore not be used in any real-world applications, it is only meant for small personal projects such as mine. 
//! 
//! # Usage
//! When using this crate it is important that you first [preprocess](preprocessing) your data  which can be given in either string form or as a vector of `u8` bytes.
//! The algorithm will return a [BigInt](crypto_bigint::Uint) from the external [crypto_bigint] crate.
//! If this is not your preferred data type there are functions in [conversions] to transform this output to a list of either u64 or u32 words.
//! 
//! # Example
//! ```
//! use jisp_sha2::preprocessing::sha256_preprocessing;
//! use jisp_sha2::sha256::sha_256;
//! use crypto_bigint::U256;
//! 
//! let input_str = "abc";
//! 
//! let hex = sha256_preprocessing("abc");
//! let hash = sha_256(hex);
//! 
//! let expected = 
//!     U256::from_be_hex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
//! assert_eq!(hash, expected);
//! ```



pub mod preprocessing;
pub use preprocessing::sha256_preprocessing;
pub use preprocessing::sha512_preprocessing;
pub use preprocessing::custom_preprocessing;
pub use sha256::sha_256;
pub use sha256::sha_224;
pub use sha512::sha_512;
pub use sha512::sha_384;

pub mod conversions;
pub mod printer;
pub mod constants;
pub mod sha256;
pub mod sha512;