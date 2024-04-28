use crypto_bigint::U512;
use crate::conversions::to_u64_words;


pub fn print_blocks(v: &Vec<U512>, split:bool) -> String {
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
            res += &format!("{:x?} {:x?} ", i/m, i%m);
        } else {
            res += &format!("{:x?}", i);
        }
        
    }
    res
}