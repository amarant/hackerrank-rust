use std::io;
use std::io::BufRead;

fn candies(ratings: &Vec<i32>) -> i32 {
	let mut candies = vec![1i32; ratings.len()];
	for i in 1..ratings.len() {
		if ratings[i] > ratings[i-1] {
			candies[i] = candies[i-1] + 1;
		}
	}
	for i in (1..ratings.len()).rev() {
		if ratings[i-1] > ratings[i] {
			candies[i-1] = std::cmp::max(candies[i-1], candies[i] + 1);
		}
	}
	candies.iter().fold(0, |sum, i| sum + *i)
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line_count = lines.next().unwrap().unwrap();
    let count = line_count.parse::<usize>().unwrap();
    let ratings: Vec<i32> = lines.take(count)
        .map(|value| value.unwrap().parse::<i32>().unwrap())
        .collect();
    let change_count = candies(&ratings);
    println!("{}", change_count);
}

#[cfg(test)]
mod tests {
    #[test]
    fn candies_test0() {
    	assert_eq!(4, super::candies(&vec![1, 2, 2]));
    }
    #[test]
    fn candies_test1() {
    	assert_eq!(19, super::candies(&vec![2, 4, 2, 6, 1, 7, 8, 9, 2, 1]));
    }
    #[test]
    fn candies_incr_decr() {
    	assert_eq!(16, super::candies(&vec![1, 2, 3, 4, 3, 2, 1]));
	}
    #[test]
    fn candies_decr_incr_decr() {
    	assert_eq!(9, super::candies(&vec![6, 5, 4, 5, 4]));
	}
}
