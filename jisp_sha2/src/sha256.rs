use std::ops::{BitXor, BitAnd};

use crypto_bigint::{U512,U256};
use crate::conversions;
use crate::constants::{Constants, Sha256, Sha224};


pub fn sha_256(m:Vec<U512>) -> U256 {
    sha256_internal::<Sha256>(m)
}

pub fn sha_224(m:Vec<U512>) -> [u32;7] {
    let hash = sha256_internal::<Sha224>(m);
    let words = conversions::to_u32_words(hash);
    let mut truncated_words = [0u32; 7];
    for i in 0..7 {
        truncated_words[i] = words[i];
    } 
    return truncated_words;
}


fn sha256_internal<C:Constants<64,u32>>(msg:Vec<U512>) -> U256 {
    let mut hash = C::initial_hash();
    for block in msg {
        let registers = sha256_compression::<C>(&hash, block);
        for i in 0..8 {
            hash[i] = hash[i].wrapping_add(registers[i]);
        }
    }
    conversions::from_u32_words(&hash.to_vec())
    
}

fn sha256_compression<C:Constants<64,u32>>(intermediate_hash:&[u32;8], msg:U512) -> [u32;8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = intermediate_hash.clone();
    let constants_k = C::constant_words();
    let expanded_blocks = sha256_message_schedule(msg);

    for j in 0..64 {
        let T1 = wrapping_sum(vec![
            h, sigma_l1(e), ch(e,f,g), constants_k[j], expanded_blocks[j] 
        ]);
        let T2 = sigma_l0(a).wrapping_add(maj(a,b,c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
    }

    [a, b, c, d, e, f, g, h]
}

// ------ Six Logical Functions -------

fn ch(x:u32, y:u32, z:u32) -> u32 {
    let x_and_y = x.bitand(y);
    let x_and_z = inv(x).bitand(z);
    return x_and_y.bitxor(x_and_z);
}

fn maj(x:u32, y:u32, z:u32) -> u32 {
    let x_and_y = x.bitand(y);
    let y_and_z = y.bitand(z);
    let x_and_z = x.bitand(z);
    return x_and_y.bitxor(y_and_z).bitxor(x_and_z);
}

fn sigma_l0(x:u32) -> u32 {
    let s2 = x.rotate_right(2);
    let s13 = x.rotate_right(13);
    let s22 = x.rotate_right(22);
    return s2.bitxor(s13).bitxor(s22);
}

fn sigma_l1(x:u32) -> u32 {
    let s6 = x.rotate_right(6);
    let s11 = x.rotate_right(11);
    let s25 = x.rotate_right(25);
    return s6.bitxor(s11).bitxor(s25);

}

fn sigma_s0(x:u32) -> u32 {
    let s7 = x.rotate_right(7);
    let s18 = x.rotate_right(18);
    let r3 = x >> 3;
    return s7.bitxor(s18).bitxor(r3);
}

fn sigma_s1(x:u32) -> u32 {
    let s17 = x.rotate_right(17);
    let s19 = x.rotate_right(19);
    let r10 = x >> 10;
    return s17.bitxor(s19).bitxor(r10);
}

// --------- addendum -------------
fn inv(x:u32) -> u32 {
    x.bitxor(u32::MAX)
}

fn xor_sum(v:Vec<u32>) -> u32 {
    let mut res = 0u32;
    for u in v {
        res = res.bitxor(u);
    }
    return res;
}

fn wrapping_sum(v:Vec<u32>) -> u32 {
    let mut res = 0u32;
    for u in v {
        res = res.wrapping_add(u);
    }
    return res;
}


// ------------------ Message Schedule ----------------

fn sha256_message_schedule(m:U512) -> [u32;64] {
    //word vector of length 16
    let m = conversions::to_u32_words(m); 
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = m[i];
    }

    for i in 16..64 {
        let mut w_i = sigma_s1(w[i - 2]);
        w_i = w_i.wrapping_add(w[i - 7]);
        w_i = w_i.wrapping_add(sigma_s0(w[i - 15]));
        w_i = w_i.wrapping_add(w[i - 16]);
        w[i] = w_i;
    }

    return w;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_sigma_s0() {
        let y = sigma_s0(1);
        let expected = (1 << 25) + (1 << 14);
        assert_eq!(y, expected);
    }

    #[test]
    fn simple_sigma_s1() {
        let y = sigma_s1(1 << 31);
        let expected = 0b00000000_00100000_01010000_00000000;
        assert_eq!(y,expected);
    }
}