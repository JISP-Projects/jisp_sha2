//! simple functions for printing lists of bytes with spaces inbetween every 4 or 8 bytes
use crypto_bigint::Uint;
use crate::conversions::to_u64_words;

///Splits either every 4 bytes or every 8 bytes based on the `split4` boolean
pub fn print_blocks<const BLOCK:usize>(v: &Vec<Uint<BLOCK>>, split4:bool) -> String {
    let mut res = "".to_owned();
    for n in v {
        let words = to_u64_words(*n);
        res += &print_word_string(&words.to_vec(), split4);
    }
    res
}

///Splits either every 4 bytes or every 8 bytes based on the `split4` boolean
pub fn print_word_string(v:&Vec<u64>, split4:bool) -> String {
    let mut res = "".to_owned();
    let m : u64 = 1 << 32;  
    for i in v.iter() {
        if split4 {
            res += &format!("{:08x?} {:08x?} ", i/m, i%m);
        } else {
            res += &format!("{:016x?} ", i);
        }
        
    }
    res
}

//splits after every u32 word. Which is once every 4 bytes.
pub fn print_u32_word_string(v:&Vec<u32>) -> String {
    let mut res = "".to_owned();
    for i in v.iter() {
        res += &format!("{:08x?} ", i);
    }
    res
}