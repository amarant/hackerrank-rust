use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let num = line.parse::<usize>().unwrap();
    let mut hashes = String::with_capacity(num);
    let mut spaces = String::from_utf8(vec![b' '; num]).unwrap();
    for _ in 1..num+1 {
        hashes.push('#');
        spaces.pop();
        println!("{}{}", spaces, hashes);
    }
}
