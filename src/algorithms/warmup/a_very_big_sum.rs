use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let count = line.trim().parse().unwrap();
    
    let numbers = stdin.lock().lines().next().unwrap().unwrap();
    let v: Vec<&str> = numbers.trim().split(' ').collect();
    let mut sum = 0u64;
    for i in 0..count {
        sum += v[i].parse::<u64>().unwrap();
    }
    println!("{}", sum);
}
