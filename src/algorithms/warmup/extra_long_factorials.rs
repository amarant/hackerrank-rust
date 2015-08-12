extern crate num;

use std::io;
use std::io::BufRead;
use num::bigint::{BigUint, ToBigUint};
use num::One;

fn big_factorial(n: usize) -> BigUint {
    if n == 0 {
        return One::one();
    }
    n.to_biguint().unwrap() * big_factorial(n - 1)
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    let res = big_factorial(n);
    println!("{}", res);
}
