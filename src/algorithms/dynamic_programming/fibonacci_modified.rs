extern crate num;

use std::io;
use std::io::BufRead;
use num::BigUint;
use std::mem::replace;

fn fib_mod(a: BigUint, b: BigUint, n: usize) -> BigUint {
    let mut tn = a;
    let mut tn1 = b;
    for _ in 1..(n-1) {
        let tn2 = &tn1*&tn1 + tn;
        tn = replace(&mut tn1, tn2);
    }
    tn1
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<&str> = line.trim().split(' ').collect();
    let a = numbers[0].parse::<BigUint>().unwrap();
    let b = numbers[1].parse::<BigUint>().unwrap();
    let n = numbers[2].parse::<usize>().unwrap();
    let res = fib_mod(a, b, n);
    println!("{}", res);
}
