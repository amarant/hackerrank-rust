use std::io;
use std::io::BufRead;
use std::cmp::Ordering;

fn candies(ratings: &Vec<i32>) -> i32 {
	//println!("ratings:");
	//println!("{:?}", ratings);
	let mut last_candies = 1i32;
	let mut last_rating = ratings[0];
	let mut candies = Vec::with_capacity(ratings.len());
	candies.push(1);
	for index in 1..ratings.len() {
		let current_rating = ratings[index];
		match (current_rating).cmp(&last_rating) {
            Ordering::Less => {
            	if last_candies == 1 {
            		//println!("candies:");
            		//println!("{:?}", candies);

					let mut next_rating = current_rating;
					let mut next_candy = 1;
					for backward_index in (0..index).rev() {
	            		let mut current_candy = candies[backward_index];
            			let current_rating = ratings[backward_index];
						
						// println!("rating:{:?}, candy:{:?}", 
						// 	current_rating, current_candy);
						if current_rating > next_rating
							&& current_candy <= next_candy {
								current_candy = next_candy + 1;
							}
						candies[backward_index] = current_candy;	
				        next_candy = current_candy;
            			next_rating = current_rating;
					}
            	}
            	last_candies = 1;
            },
            Ordering::Equal => {
            	last_candies = 1;
            },
            Ordering::Greater => {
				last_candies += 1;
            },
        }
    	candies.push(last_candies);
		last_rating = current_rating;
	}
	//println!("candies:");
	//println!("{:?}", candies);
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
