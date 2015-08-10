use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line0 = stdin.lock().lines().next().unwrap().unwrap();
    let line0_split : Vec<&str> = line0.split(' ').collect();
    let actual_day = line0_split[0].parse::<i32>().unwrap();
    let actual_month = line0_split[1].parse::<i32>().unwrap();
    let actual_year = line0_split[2].parse::<i32>().unwrap();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let line1_split : Vec<&str> = line1.split(' ').collect();
    let expected_day = line1_split[0].parse::<i32>().unwrap();
    let expected_month = line1_split[1].parse::<i32>().unwrap();
    let expected_year = line1_split[2].parse::<i32>().unwrap();
    let mut fine = 0i32;
    if actual_year == expected_year {
        if actual_month == expected_month {
            if actual_day > expected_day {
                fine = 15 * (actual_day - expected_day);
            }
        } else if actual_month > expected_month {
            fine = 500 * (actual_month - expected_month);
        }
    } else if actual_year > expected_year {
        fine = 10000;
    }
    println!("{}", fine);
}
