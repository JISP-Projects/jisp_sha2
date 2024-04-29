use crypto_bigint::{U512, Uint};
use crate::conversions::to_u64_words;


pub fn print_blocks<const BLOCK:usize>(v: &Vec<Uint<BLOCK>>, split:bool) -> String {
    let mut res = "".to_owned();
    for n in v {
        let words = to_u64_words(*n);
        res += &print_word_string(&words.to_vec(), split);
    }
    res
}
pub fn print_word_string(v:&Vec<u64>, split:bool) -> String {
    let mut res = "".to_owned();
    let m : u64 = 1 << 32;  
    for i in v.iter() {
        if split {
            res += &format!("{:08x?} {:08x?} ", i/m, i%m);
        } else {
            res += &format!("{:016x?} ", i);
        }
        
    }
    res
}

pub fn print_u32_word_string(v:&Vec<u32>) -> String {
    let mut res = "".to_owned();
    for i in v.iter() {
        res += &format!("{:08x?} ", i);
    }
    res
}