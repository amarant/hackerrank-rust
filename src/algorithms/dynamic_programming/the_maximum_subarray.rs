use std::io;
use std::io::BufRead;
use std::cmp;

fn max_subarray(values: &Vec<i32>) -> i32 {
    let mut max_ending_here = values[0];
    let mut max_so_far = max_ending_here;
    for value in values.iter().skip(1) {
        max_ending_here = cmp::max(*value, max_ending_here + value);
        max_so_far = cmp::max(max_so_far, max_ending_here);
    }
    max_so_far
}

fn max_disjoint_subarray(values: &Vec<i32>) -> i32 {
    // with rust 1.2+ it is possible to do a partition with
    // Vec<i32> and remove those references but not with
    // the current hackerran rust 1.1-
    let (strict_pos, neg): (Vec<_>, Vec<_>) = values.into_iter()
        .partition(|&value| *value > 0);
    if strict_pos.is_empty() {
        **neg.iter().max().unwrap()
    } else {
        strict_pos.iter().fold(0, |sum, i| sum + *i)
    }

}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let t = first_line.trim().parse::<usize>().unwrap();
    let mut lines_enum = lines.take(t*2);
    let mut results = Vec::<(i32, i32)>::new();
    while let Some(line_count) = lines_enum.next() {
        let count = line_count.unwrap().trim().parse::<usize>().unwrap();
        let line_values = lines_enum.next().unwrap().unwrap();
        let values: Vec<i32> = line_values.trim().split(' ')
            .map(|value| value.parse::<i32>().unwrap())
            .take(count)
            .collect();
        results.push((max_subarray(&values), max_disjoint_subarray(&values)));
    }

    for result in results.iter() {
        println!("{} {}", result.0, result.1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_subarray() {
        assert_eq!(10, super::max_subarray(&vec![1, 2, 3, 4]));
        assert_eq!(10, super::max_subarray(&vec![2, -1, 2, 3, 4, -5]));
        assert_eq!(-1, super::max_subarray(&vec![-1]));
        assert_eq!(-1, super::max_subarray(&vec![-2, -1]));
    }

    #[test]
    fn max_disjoint_subarray() {
        assert_eq!(10, super::max_disjoint_subarray(&vec![1, 2, 3, 4]));
        assert_eq!(11, super::max_disjoint_subarray(&vec![2, -1, 2, 3, 4, -5]));
        assert_eq!(-1, super::max_disjoint_subarray(&vec![-1]));
        assert_eq!(-1, super::max_disjoint_subarray(&vec![-2, -1]));
    }
}
