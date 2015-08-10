use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut hours = line[0..2].parse::<u8>().unwrap();
    let minutes = &line[3..5];
    let seconds = &line[6..8];
    let period = &line[8..9];
    if period == "A" && hours == 12 {
        hours = 0;
    }
    if period == "P" && hours != 12 {
        hours += 12;
    }
    println!("{:02}:{}:{}", hours, minutes, seconds);
}
