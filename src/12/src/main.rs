use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use util::input::get_input_string;

type Graph = HashMap<String, Vec<String>>;

#[allow(dead_code)]
fn make_graph(strings: &Vec<String>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for string in strings {
        let pair = string.split("-").collect::<Vec<&str>>();
        if let [from, to] = pair[..] {
            if from != "start" {
                match graph.entry(to.to_string()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(from.to_string());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![from.to_string()]);
                    }
                }
            }
            if to != "start" {
                match graph.entry(from.to_string()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(to.to_string());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![to.to_string()]);
                    }
                }
            }
        }
    }
    graph
}

#[allow(dead_code)]
fn print_graph(graph: &Graph) {
    for (node, children) in graph.iter() {
        print!("{}: {}\n", node, children.join(" - "));
    }
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("12");
    let graph = make_graph(&strings);
    let mut total = 0;
    let mut queue: VecDeque<(String, HashSet<String>)> = VecDeque::from([(
        String::from("start"),
        HashSet::from([String::from("start")]),
    )]);
    while !queue.is_empty() {
        let pair = queue.pop_front().unwrap();
        let from = pair.0;
        if from == "end" {
            total += 1;
            continue;
        }
        let visited = pair.1;
        for child in graph[&from].iter() {
            if !visited.contains(child) || !child.chars().all(|x| x.is_lowercase()) {
                let mut new_visited = HashSet::new();
                new_visited.clone_from(&visited);
                new_visited.insert(child.to_string());
                queue.push_back((child.to_string(), new_visited));
            }
        }
    }
    total
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("12");
    let graph = make_graph(&strings);
    let mut total = 0;
    let mut queue: VecDeque<(String, HashSet<String>, i64)> = VecDeque::from([(
        String::from("start"),
        HashSet::from([String::from("start")]),
        1,
    )]);
    let mut lower: HashSet<String> = HashSet::new();
    let mut upper: HashSet<String> = HashSet::new();
    for (v, _) in &graph {
        if v.chars().all(|x| x.is_lowercase()) {
            lower.insert(v.to_string());
        } else if v.chars().all(|x| x.is_uppercase()) {
            upper.insert(v.to_string());
        }
    }
    while !queue.is_empty() {
        let pair = queue.pop_front().unwrap();
        let from = pair.0;
        if from == "end" {
            total += 1;
            continue;
        }
        let visited = pair.1;
        let duplicated_count = pair.2;
        for child in graph[&from].iter() {
            if !visited.contains(child) || upper.contains(child) {
                let mut new_visited = HashSet::new();
                new_visited.clone_from(&visited);
                new_visited.insert(child.to_string());
                queue.push_back((child.to_string(), new_visited, duplicated_count));
            } else if visited.contains(child) && duplicated_count > 0 && lower.contains(child) {
                let mut new_visited = HashSet::new();
                new_visited.clone_from(&visited);
                new_visited.insert(child.to_string());
                queue.push_back((child.to_string(), new_visited, duplicated_count - 1));
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 3887);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 104834);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
