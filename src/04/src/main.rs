use std::collections::HashSet;
use util::input::{get_input_string, split_string};

type Board = Vec<Vec<i64>>;

fn sum_unchecked(b: &Board, s: &HashSet<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !s.contains(&b[i][j]) {
                sum += b[i][j];
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn bingo(b: &Board, s: &HashSet<i64>) -> i64 {
    for i in 0..5 {
        let mut j = 0;
        while j < 5 && s.contains(&b[i][j]) {
            j += 1;
        }
        if j == 5 {
            return sum_unchecked(b, s);
        }
        j = 0;
        while j < 5 && s.contains(&b[j][i]) {
            j += 1;
        }
        if j == 5 {
            return sum_unchecked(b, s);
        }
    }
    0
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("04");
    let orders: Vec<i64> = split_string(",", strings.first().unwrap());
    let length = strings.len();
    let mut boards = Vec::new();
    for i in (0..length).skip(1).step_by(6) {
        let board_numbers: Board = strings[i + 1..i + 6]
            .iter()
            .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
            .collect();
        boards.push(board_numbers);
    }
    let mut called_numbers = HashSet::new();
    for order in orders {
        called_numbers.insert(order);
        for board in &boards {
            let can_bingo = bingo(&board, &called_numbers);
            if can_bingo != 0 {
                return can_bingo * order;
            }
        }
    }
    panic!("Cannot find such person");
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("04");
    let orders: Vec<i64> = split_string(",", strings.first().unwrap());
    let length = strings.len();
    let mut boards = Vec::new();
    for i in (0..length).skip(1).step_by(6) {
        let board_numbers: Board = strings[i + 1..i + 6]
            .iter()
            .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
            .collect();
        boards.push(board_numbers);
    }
    let mut called_numbers = HashSet::new();
    let mut winning_sequence = Vec::new();
    for order in orders {
        called_numbers.insert(order);
        boards.retain(|board| {
            let can_bingo = bingo(&board, &called_numbers);
            if can_bingo != 0 {
                winning_sequence.push(can_bingo * order);
                return false;
            }
            true
        });
    }
    if winning_sequence.is_empty() {
        panic!("Cannot find such person");
    } else {
        winning_sequence.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 11536);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1284);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
