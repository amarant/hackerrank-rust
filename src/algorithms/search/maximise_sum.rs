#![feature(btree_range)]
#![feature(collections_bound)]

use std::io;
use std::io::BufRead;
use std::cmp;
//use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::collections::Bound::{Included};

fn modu(value: i64, modulo: i64) -> i64 {
    let ret = value % modulo;
    if ret < 0 {
        ret + modulo
    } else {
        ret
    }
}

fn maximise_sum(values: &Vec<i64>, modulo: i64) -> i64 {
    println!("values:{:?} modulo:{:?}", values, modulo);
    let mut cumulative_sum_values = Vec::with_capacity(values.len());
    let mut cumulative_sum = 0i64;
    for value in values.iter() {
        cumulative_sum = modu(cumulative_sum + value, modulo);
        cumulative_sum_values.push(cumulative_sum);
    }
    println!("cumulative_sum_values:{:?}", cumulative_sum_values);
    let mut max_so_far = 0;
    // let mut cumulative_sum_tree :BinaryHeap<i64>
    //     = BinaryHeap::with_capacity(values.len());
    let mut cumulative_sum_tree :BTreeSet<i64>
        = BTreeSet::new();
    cumulative_sum_tree.insert(0);
    for &cumulative_sum in cumulative_sum_values.iter() {
        let min0 = cumulative_sum - modulo + 1;
        let max0 = cumulative_sum - max_so_far - 1;
        let min1 = cumulative_sum + 1;
        let max1 = cumulative_sum + modulo - max_so_far;
        println!("cumulative_sum:{:?} [{:?} {:?}] [{:?} {:?}]",
            cumulative_sum,
            min0, max0, min1, max1);

        if let Some(value0) = cumulative_sum_tree.range(
                Included(&min0),
                Included(&max0)).nth(0) {
            max_so_far = cmp::max(max_so_far, modu(cumulative_sum - value0, modulo));
            println!("value0:{:?} max_so_far:{:?}", value0, max_so_far);
        }
        if let Some(value1) = cumulative_sum_tree.range(
                Included(&min1),
                Included(&max1)).nth(0) {
            max_so_far = cmp::max(max_so_far, modu(cumulative_sum - value1, modulo));
            println!("value1:{:?} max_so_far:{:?}", value1, max_so_far);
        }
        cumulative_sum_tree.insert(cumulative_sum);
    }

    max_so_far
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let t = first_line.trim().parse::<usize>().unwrap();
    let mut lines_enum = lines.take(t*2);
    let mut results = Vec::<i64>::new();
    while let Some(line_count) = lines_enum.next() {
        let count_and_modulo: Vec<i64> = line_count.unwrap().trim().split(' ')
            .map(|value| value.parse::<i64>().unwrap())
            .collect();
        let count = count_and_modulo[0] as usize;
        let modulo = count_and_modulo[1];
        let line_values = lines_enum.next().unwrap().unwrap();
        let values: Vec<i64> = line_values.trim().split(' ')
            .map(|value| value.parse::<i64>().unwrap())
            .take(count)
            .collect();
        results.push((maximise_sum(&values, modulo)));
    }

    let result = results.iter().fold(0, |sum, value| sum + value);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        assert_eq!(6, super::maximise_sum(&vec![3, 3, 9, 9, 5], 7));
    }

    #[test]
    fn big() {
        assert_eq!(1, super::maximise_sum(&vec![10e18 as i64, 1], 10));
    }
    #[test]
    fn test1() {
        assert_eq!(6, super::maximise_sum(&vec![4,5,3,10], 7));
    }
    #[test]
    fn test2() {
        assert_eq!(8, super::maximise_sum(&vec![1, 3, 10, 7, 6, 5], 10));
    }
}
