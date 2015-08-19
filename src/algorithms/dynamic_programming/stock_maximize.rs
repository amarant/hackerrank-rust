use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt::{Debug, Formatter, Error};

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

#[derive(Hash, Clone)]
struct StockPosition {
    stock_count: i32,
    money: i32,
}
impl Debug for StockPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[stocks:{} money:{}]",
            self.stock_count, self.money)
    }
}

fn stock_maximize2(stock_values: &Vec<i32>) -> i32 {
    let mut stock_positions :HashMap<i32, StockPosition> = HashMap::new();
    stock_positions.insert(0, StockPosition {stock_count: 0, money: 0});
    for stock_value in stock_values.iter() {
        let mut new_stock_position :HashMap<i32, StockPosition> = HashMap::new();
        for position in stock_positions.values() {
            let position_do_nothing = position.clone();
            insert_if_more_money(&mut new_stock_position, position_do_nothing);

            let mut position_buy = position.clone();
            position_buy.stock_count += 1;
            position_buy.money -= *stock_value;
            insert_if_more_money(&mut new_stock_position, position_buy);

            let mut position_sell = position.clone();
            position_sell.money += position.stock_count * stock_value;
            position_sell.stock_count = 0;
            insert_if_more_money(&mut new_stock_position, position_sell);
        }
        //println!("stock value:{} positions:{:?}", stock_value, new_stock_position);
        stock_positions = new_stock_position;
    }
    stock_positions.values().map(|pos| pos.money).max().unwrap()
}

fn insert_if_more_money(
    stock_positions: &mut HashMap<i32, StockPosition>,
    position: StockPosition) {
    match stock_positions.entry(position.stock_count) {
        Vacant(entry) => {entry.insert(position); ()},
        Occupied(mut entry) => {
            if entry.get().money < position.money {
                entry.insert(position);
            }
        },
    }
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