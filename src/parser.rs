use crypto_bigint::{Uint, U1024, U512};
use crate::conversions;

pub fn string_to_encoding(s: &str) -> Vec<u8> {
    s.as_bytes().into()
}

pub fn sha256_preprocessing(s: &str) -> Vec<U512> {
    preprocessing::<8, 1>(s)
}

pub fn sha512_preprocessing(s: &str) -> Vec<U1024> {
    preprocessing::<16, 2>(s)
}

pub fn preprocessing<const BLOCK: usize, const SUFFIX: usize>(s: &str) -> Vec<Uint<BLOCK>> {
    let bytes = string_to_encoding(s);
    let l = 8 * bytes.len();
    let mut l_vec = [0 as u64; SUFFIX];
    l_vec[SUFFIX - 1] = l as u64;

    let words = byte_padding(&bytes);
    word_padding(&words, l_vec)
}

/// Packs 64 bit words into 512 bit blocks by padding the message with 0s and adding the suffix l (usually the length of the message) at the end of the final block.
/// # Examples
/// The word padding function completely trusts you that the length you give was the original length of the message
/// ```
/// use jisp_sha2::parser::word_padding;
/// use crypto_bigint::{U512, CheckedAdd};
///
/// let v = vec![1];
/// let len = (v.len()*8) as u64;
///
/// let res = word_padding::<8,1>(&v, [len]);
/// let expected = U512::ONE.shl(448).checked_add(&U512::from(8u8)).unwrap();
/// println!("{:?}",res);
///
/// assert_eq!(res, vec![expected])
/// ```
pub fn word_padding<const BSIZE: usize, const SUFFIX: usize>(
    v: &Vec<u64>,
    l: [u64; SUFFIX],
) -> Vec<Uint<BSIZE>> {
    let mut blocks = Vec::new();
    let mut block = [0u64; BSIZE];
    let mut block_index = 0;

    if SUFFIX > BSIZE {
        panic!(
            "Size SUFFIX:{} is larger than block size BSIZE: {}",
            SUFFIX, BSIZE
        );
    }

    for word in v.iter() {
        block[block_index] = *word;
        block_index += 1;
        if block_index == BSIZE {
            block_index = 0;
            blocks.push(conversions::from_u64_words(&mut block));
        }
    }

    let mut i = BSIZE - SUFFIX;
    //check if length fits in remaining block space
    if i < block_index {
        blocks.push(conversions::from_u64_words(&mut block));
    }

    //append l to the end of the final block and push
    for l_elem in l {
        block[i] = l_elem;
        i += 1;
    }
    blocks.push(conversions::from_u64_words(&mut block));

    return blocks;
}
/// Merges groups of 8 bytes into 64 bit words
/// We extend the length of the list of bytes by adding a 1 bit at the end and enough 0s such that we have a multiple of 64 bits
///  # Example
/// ```
/// use jisp_sha2::parser::byte_padding;
///
/// let mut v = vec![1,2,3];
///
/// let res = byte_padding(&v);
/// let expected = 0x0102_0380_0000_0000u64;
///
/// assert_eq!(res, vec![expected])
/// ```
///
///
/// the 1 packing at the end means we add an extra u64 if your message is already a multiple of 8 bytes
/// ```
/// use jisp_sha2::parser::byte_padding;
///
/// let mut v = vec![0;8];
/// v[7] = 5;
///
/// let res = byte_padding(&v);
///
/// assert_eq!(res,vec![5, 1 << 63])
/// ```
pub fn byte_padding(v: &Vec<u8>) -> Vec<u64> {
    let mut words = Vec::new();
    let mut word = [0; 8];
    let mut byte_i = 0;
    for u in v.iter() {
        word[byte_i] = *u;
        byte_i += 1;
        if byte_i == 8 {
            byte_i = 0;
            words.push(bytes_to_u64(word));
            word = [0; 8];
        }
    }
    //We do not know if we completed the last word when we ended
    //add 1000 0000 to the result we have
    word[byte_i] = 0x80u8;
    words.push(bytes_to_u64(word));

    return words;
}

pub fn bytes_to_u64(v: [u8; 8]) -> u64 {
    let mut res: u64 = 0;
    for i in v.iter() {
        res = res << 8;
        res += *i as u64;
    }
    res
}
