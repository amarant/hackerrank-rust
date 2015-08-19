use std::io;
use std::io::BufRead;

fn stock_maximize(stock_values: &Vec<i64>) -> i64 {
    let mut money = 0i64;
    let mut current_max = *stock_values.last().unwrap();
    for i in (0..stock_values.len()).rev() {
        if stock_values[i] <= current_max {
            money += current_max - stock_values[i];
        } else {
            current_max = stock_values[i];
        }
    }
    money
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let t = first_line.trim().parse::<usize>().unwrap();
    let mut lines_enum = lines.take(t*2);
    let mut results = Vec::<i64>::with_capacity(t);
    while let Some(line_count) = lines_enum.next() {
        let count = line_count.unwrap().trim().parse::<usize>().unwrap();
        let line_values = lines_enum.next().unwrap().unwrap();
        let values: Vec<i64> = line_values.trim().split(' ')
            .map(|value| value.parse::<i64>().unwrap())
            .take(count)
            .collect();
        results.push(stock_maximize(&values));
    }

    for result in results.iter() {
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample1() {
        assert_eq!(0, super::stock_maximize(&vec![5, 3, 2]));
    }

    #[test]
    fn sample2() {
        assert_eq!(197, super::stock_maximize(&vec![1, 2, 100]));
    }

    #[test]
    fn sample3() {
        assert_eq!(3, super::stock_maximize(&vec![1, 3, 1, 2]));
    }

    //&(1..250).collect()
    #[test]
    fn big() {
        super::stock_maximize(&(1..40000).collect());
    }
}