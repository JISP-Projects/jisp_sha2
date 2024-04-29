use std::ops::{BitXor, BitAnd};

use crypto_bigint::{U512,U1024};
use crate::conversions;
use crate::constants::{Constants, Sha512};


pub fn sha_512(m:Vec<U1024>) -> U512 {
    sha512_internal::<Sha512>(m)
}


fn sha512_internal<C:Constants<80,u64>>(msg:Vec<U1024>) -> U512 {
    let mut hash = C::initial_hash();
    for block in msg {
        let registers = sha512_compression::<C>(&hash, block);
        for i in 0..8 {
            hash[i] = hash[i].wrapping_add(registers[i]);
        }
    }
    conversions::from_u64_words(&mut hash)
    
}

fn sha512_compression<C:Constants<80,u64>>(intermediate_hash:&[u64;8], msg:U1024) -> [u64;8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = intermediate_hash.clone();
    let constants_k = C::constant_words();
    let expanded_blocks = sha512_message_schedule(msg);

    for j in 0..80 {
        let T1 = wrap_sum(vec![
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

fn ch(x:u64, y:u64, z:u64) -> u64 {
    let x_and_y = x.bitand(y);
    let x_and_z = inv(x).bitand(z);
    return x_and_y.bitxor(x_and_z);
}

fn maj(x:u64, y:u64, z:u64) -> u64 {
    let x_and_y = x.bitand(y);
    let y_and_z = y.bitand(z);
    let x_and_z = x.bitand(z);
    return xor_sum(vec![x_and_y, y_and_z, x_and_z]);
}

fn sigma_l0(x:u64) -> u64 {
    return xor_sum(vec![s(x,28), s(x,34), s(x,39)]);
}

fn sigma_l1(x:u64) -> u64 {
    return xor_sum(vec![s(x,14), s(x,18), s(x,41)]);
}

fn sigma_s0(x:u64) -> u64 {
    return xor_sum(vec![s(x,1), s(x,8), r(x,7)]);
}

fn sigma_s1(x:u64) -> u64 {
    return xor_sum(vec![s(x,19), s(x, 61), r(x,6)])
}

// --------- addendum -------------
fn inv(x:u64) -> u64 {
    x.bitxor(u64::MAX)
}

fn xor_sum(v:Vec<u64>) -> u64 {
    let mut res = 0u64;
    for u in v {
        res = res.bitxor(u);
    }
    return res;
}

fn wrap_sum(v:Vec<u64>) -> u64 {
    let mut res = 0u64;
    for u in v {
        res = res.wrapping_add(u);
    }
    return res;
}

//Renamed functions to fit with the SHA-2 design document
fn s(uint:u64, rot:u8) -> u64 {
    uint.rotate_right(rot as u32)
} 
fn r(uint:u64, shift:u8) -> u64 {
    uint >> shift
}


// ------------------ Message Schedule ----------------

fn sha512_message_schedule(m:U1024) -> [u64;80] {
    //word vector of length 16
    let m = conversions::to_u64_words(m); 
    let mut w = [0u64; 80];
    for i in 0..16 {
        w[i] = m[i];
    }

    for i in 16..80 {
        w[i] = wrap_sum(vec![
            sigma_s1(w[i - 2]),
            w[i - 7],
            sigma_s0(w[i - 15]),
            w[i - 16]
        ]);
    }

    return w;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_sigma_l0() {
        let y = sigma_l0(1);
        let expected = (1 << 36) + (1 << 30) + (1 << 27);
        assert_eq!(y, expected);
    }

    #[test]
    fn simple_sigma_s0() {
        let y = sigma_s0(1 << 10);
        let expected = (1 << 9) + (1 << 2) + (1 << 3);
        assert_eq!(y,expected);
    }
}