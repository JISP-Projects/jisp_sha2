use crypto_bigint::Uint;

/// Takes an array of words and transforms it into a big integer. Resets the array to all 0s in the process
/// # Examples
/// ```
/// use jisp_sha2::conversions::from_u64_words;
/// use crypto_bigint::U128;
///
/// let mut words = [0, 1];
/// let block = from_u64_words(&mut words);
///
/// assert_eq!(block, U128::from(1u8));
/// assert_eq!(words, [0;2]);
/// ```
pub fn from_u64_words<const BLOCK: usize>(words: &mut [u64; BLOCK]) -> Uint<BLOCK> {
    words.reverse();
    let res = Uint::<BLOCK>::from_words(*words);
    *words = [0; BLOCK];
    return res;
}

/// Seperates a big integer into its u64 words. It does this in big endian oder instead of the default little endian order i.e. the words are ordered most significant word first
/// # Examples
/// ```
/// use jisp_sha2::conversions::to_u64_words;
/// use crypto_bigint::U128;
///
/// let words = to_u64_words(U128::from(1u8));
///
/// assert_eq!(words, [0, 1]);
/// ```
pub fn to_u64_words<const BLOCK: usize>(u: Uint<BLOCK>) -> [u64; BLOCK] {
    let mut words = u.as_words().clone();
    words.reverse();
    return words;
}

/// Takes an array of u32 words and transforms it into a big integer. Panics if the length of the vector is not 2 times the block size
/// # Examples
/// ```
/// use jisp_sha2::conversions::from_u32_words;
/// use crypto_bigint::U64;
///
/// let mut words = vec![0, 1];
/// let block = from_u32_words(&words);
///
/// assert_eq!(block, U64::from(1u8));
/// ```
pub fn from_u32_words<const BLOCK: usize>(words_vec: &Vec<u32>) -> Uint<BLOCK> {
    if words_vec.len() != 2 * BLOCK {
        panic!(
            "Length of words_vec: {}\n is not 2 times the BLOCK length: {}",
            words_vec.len(),
            BLOCK
        );
    }

    let mut words = [0u64; BLOCK];
    for i in 0..BLOCK {
        let upper = (words_vec[2 * i] as u64) << 32;
        let lower = words_vec[2 * i + 1] as u64;
        words[i] = upper + lower;
    }
    words.reverse();
    let res = Uint::<BLOCK>::from_words(words);
    return res;
}

/// Seperates a big integer into its u32 words. It does this in big endian oder instead of the default little endian order i.e. the words are ordered most significant word first.
/// Note that the length of the resulting vector will be twice the "block length" since the block length is measured in multiples of 64 bits.
/// # Examples
/// ```
/// use jisp_sha2::conversions::to_u32_words;
/// use crypto_bigint::U64;
///
/// let words = to_u32_words(U64::from(1u8));
///
/// assert_eq!(words, [0, 1]);
/// ```
pub fn to_u32_words<const BLOCK: usize>(u: Uint<BLOCK>) -> Vec<u32> {
    let mut words = u.as_words().clone();
    words.reverse();
    let mut split = vec![0; BLOCK * 2];

    for i in 0..BLOCK {
        let upper: u32 = (words[i] >> 32) as u32;
        let lower: u32 = words[i] as u32;
        split[2 * i] = upper;
        split[2 * i + 1] = lower;
    }

    return split.to_vec();
}
