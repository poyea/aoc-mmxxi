use std::collections::HashMap;
use util::input::get_input_string;

type Pair = (char, char);
type ConversionHashMap = HashMap<Pair, char>;
type PairCountHashMap = HashMap<Pair, i64>;

#[allow(dead_code)]
fn iterate(iterations: i64) -> i64 {
    let strings = get_input_string("14");
    let template = strings.first().unwrap().chars().collect::<Vec<_>>();
    let mut char_map: ConversionHashMap = HashMap::new();
    for string in &strings[2..] {
        let elements = string.chars().collect::<Vec<_>>();
        char_map
            .entry((elements[0], elements[1]))
            .or_insert(elements[6]);
    }
    let mut pair_counter: PairCountHashMap = HashMap::new();
    for window in template.windows(2) {
        if let [px, py] = &window[..] {
            *pair_counter.entry((*px, *py)).or_insert(0) += 1;
        }
    }
    for _ in 0..iterations {
        let mut tmp_map: PairCountHashMap = HashMap::new();
        for ((px, py), mapped_to) in &char_map {
            if let Some(num) = pair_counter.get(&(*px, *py)) {
                *tmp_map.entry((*px, *mapped_to)).or_insert(0) += num;
                *tmp_map.entry((*mapped_to, *py)).or_insert(0) += num;
            }
        }
        pair_counter = tmp_map;
    }
    let mut char_counter: HashMap<char, i64> = HashMap::new();
    *char_counter.entry(*template.first().unwrap()).or_insert(0) += 1;
    *char_counter.entry(*template.last().unwrap()).or_insert(0) += 1;
    for ((px, py), num) in pair_counter {
        *char_counter.entry(px).or_insert(0) += num;
        *char_counter.entry(py).or_insert(0) += num;
    }
    (char_counter.values().max().unwrap() - char_counter.values().min().unwrap()) / 2
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    iterate(10)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    iterate(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 2891);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 4607749009683);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
