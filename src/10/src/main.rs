use std::collections::{HashMap, VecDeque};
use util::input::get_input_string;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("10");
    let open: Vec<char> = vec!['[', '{', '(', '<'];
    let score: HashMap<char, i64> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let convert: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);
    let mut total = 0;
    for string in strings {
        let mut stack: VecDeque<char> = VecDeque::new();
        for ch in string.chars() {
            if open.contains(&ch) {
                stack.push_back(ch);
            } else {
                if !stack.is_empty() && stack.back().unwrap() == &convert[&ch] {
                    stack.pop_back();
                } else {
                    total += score[&ch];
                    break;
                }
            }
        }
    }
    total
}

fn compute_scores(incomplete: &mut Vec<Vec<char>>, convert: &HashMap<char, char>) -> i64 {
    let score: HashMap<char, i64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut results: Vec<i64> = Vec::new();
    for line in incomplete {
        line.reverse();
        let mut sub_total = 0;
        for ch in line {
            sub_total *= 5;
            sub_total += score[&convert[ch]];
        }
        results.push(sub_total);
    }
    results.sort();
    results[results.len() >> 1]
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("10");
    let open: Vec<char> = vec!['[', '{', '(', '<'];
    let convert: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);
    let mut incomplete: Vec<Vec<char>> = Vec::new();
    for string in strings {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut can_add = true;
        for ch in string.chars() {
            if open.contains(&ch) {
                stack.push_back(ch);
            } else {
                if !stack.is_empty() && stack.back().unwrap() == &convert[&ch] {
                    stack.pop_back();
                } else {
                    can_add = false;
                    break;
                }
            }
        }
        if !stack.is_empty() && can_add {
            incomplete.push(Vec::from(stack));
        }
    }
    compute_scores(&mut incomplete, &convert)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 299793);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 3654963618);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
