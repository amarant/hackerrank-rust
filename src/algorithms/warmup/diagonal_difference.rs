use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let count = line.trim().parse().unwrap();
    
    let mut sum_diag = 0i64;
    let mut sum_inv_diag = 0i64;
    for i in 0..count {
        let numbers = stdin.lock().lines().next().unwrap().unwrap();
        let v: Vec<&str> = numbers.trim().split(' ').collect();
        sum_diag += v[i].parse::<i64>().unwrap();
        sum_inv_diag += v[count - i - 1].parse::<i64>().unwrap();
    }
    println!("{}", (sum_diag - sum_inv_diag).abs());
}
