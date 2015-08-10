use std::io;
use std::io::BufRead;

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let count = line.trim().parse().unwrap();
    
    for _ in 0..count {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let v: Vec<&str> = line.trim().split(' ').collect();
        let x = v[0].parse().unwrap();
        let y = v[1].parse().unwrap();
        println!("{}", add(x, y));
    }
}
