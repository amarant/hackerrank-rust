use std::io;
use std::io::BufRead;

fn change_count(amount: usize, coins: &Vec<u8>) -> i64 {
    let coins_count = coins.iter().count();
    let mut table = vec![vec![0i64; coins_count+1]; amount+1];
    for i in 0..amount+1 {
        for j in 0..coins_count+1 {
            if i == 0 {
                table[i][j] = 1;
            } else if j == 0 {
                table[i][j] = 0;
            } else if coins[j-1] > i as u8 {
                table[i][j] = table[i][j-1];
            } else {
                table[i][j] = table[(i as u8 - coins[j-1]) as usize][j]
                    + table[i][j-1];
            }
        }
    }
    table[amount][coins_count]
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let line_counts = stdin.lock().lines().next().unwrap().unwrap();
    let line_coins = stdin.lock().lines().next().unwrap().unwrap();
    let counts: Vec<&str> = line_counts.trim().split(' ').take(2).collect();
    let amount = counts[0].parse::<usize>().unwrap();
    let coin_count = counts[1].parse::<usize>().unwrap();
    let coins: Vec<u8> = line_coins.trim().split(' ')
        .map(|value| value.parse::<u8>().unwrap())
        .take(coin_count)
        .collect();
    let change_count = change_count(amount, &coins);
    println!("{}", change_count);
}

#[cfg(test)]
mod tests {
    #[test]
    fn change_count() {
        assert_eq!(4, super::change_count(4, &vec![1, 2, 3]));
        assert_eq!(5, super::change_count(10, &vec![2, 5, 3, 6]));
        assert_eq!(230793554364680, super::change_count(250, 
            &(1..250).collect()));
    }
}
