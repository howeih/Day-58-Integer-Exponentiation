extern crate num_bigint as bigint;
extern crate rand;

use bigint::{BigInt, ToBigInt};

fn power(x: i64, y: i64) -> BigInt {
    let mut x = x.to_bigint().unwrap();
    let mut y = y.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    let zero = 0.to_bigint().unwrap();
    let mut p = one.clone();
    while y > zero {
        p = if &y & &one > zero {
            &p * &x
        } else {
            p
        };
        x = &x * &x;
        y >>= 1;
    }
    p
}

fn main() {
    let z = power(2, 100);
    let result = BigInt::parse_bytes(b"1267650600228229401496703205376", 10).unwrap();
    assert_eq!(z, result);
}
