use std::io;
use std::io::BufRead;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let count = line.trim().parse().unwrap();
    
    let numbers = stdin.lock().lines().next().unwrap().unwrap();
    let v: Vec<&str> = numbers.trim().split(' ').collect();
    let mut pos_count = 0i32;
    let mut neg_count = 0i32;
    let mut zero_count = 0i32;
    for i in 0..count {
        let val = v[i].parse::<i64>().unwrap();
        match val.cmp(&0) {
            Ordering::Less => neg_count += 1,
            Ordering::Equal => zero_count += 1,
            Ordering::Greater => pos_count += 1,
        }
    }
    println!("{:.3}\n{:.3}\n{:.3}", 
        pos_count as f32/count as f32,
        neg_count as f32/count as f32,
        zero_count as f32/count as f32);
}
