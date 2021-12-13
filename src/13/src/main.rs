use std::cmp::max;
use std::collections::{HashMap, HashSet};
use util::input::{get_input_string, split_string};

#[allow(dead_code)]
struct PaperInfo {
    points: Vec<(i64, i64)>,
    folds: Vec<(char, i64)>,
}

#[allow(dead_code)]
fn make_paper_info(strings: &Vec<String>) -> PaperInfo {
    let mut points: Vec<(i64, i64)> = Vec::new();
    let mut folds: Vec<(char, i64)> = Vec::new();
    for string in strings {
        if string.starts_with("fold") {
            let pair = string
                .split(" ")
                .last()
                .unwrap()
                .split("=")
                .collect::<Vec<_>>();
            folds.push((pair[0].chars().nth(0).unwrap(), pair[1].parse().unwrap()));
        } else if string.len() > 0 {
            let pair = split_string(",", &string);
            points.push((pair[0], pair[1]));
        }
    }
    PaperInfo {
        points: points,
        folds: folds,
    }
}

#[allow(dead_code)]
fn folder(axis: char, unit: i64, points: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut new_points = HashSet::new();
    if axis == 'x' {
        for (px, py) in points {
            if px < unit {
                new_points.insert((px, py));
                continue;
            }
            new_points.insert((2 * unit - px, py));
        }
    } else {
        for (px, py) in points {
            if py < unit {
                new_points.insert((px, py));
                continue;
            }
            new_points.insert((px, 2 * unit - py));
        }
    }
    new_points.into_iter().collect::<Vec<(i64, i64)>>()
}

#[allow(dead_code)]
fn printer(points: Vec<(i64, i64)>) -> String {
    let (mut max_x, mut max_y) = (0, 0);
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    for (px, py) in points {
        *map.entry((py, px)).or_insert('.') = '#';
        max_x = max(max_x, px);
        max_y = max(max_y, py);
    }
    let mut flat_string = String::new();
    for j in 0..=max_y {
        for i in 0..=max_x {
            flat_string.push(*map.entry((j as i64, i as i64)).or_insert('.'));
        }
        flat_string.push('\n');
    }
    flat_string
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("13");
    let paper_info = make_paper_info(&strings);

    folder(
        paper_info.folds.first().unwrap().0,
        paper_info.folds.first().unwrap().1,
        paper_info.points,
    )
    .len()
    .try_into()
    .unwrap()
}

#[allow(dead_code)]
pub fn solve_part2() -> String {
    let strings = get_input_string("13");
    let mut paper_info = make_paper_info(&strings);

    for fold in paper_info.folds {
        paper_info.points = folder(fold.0, fold.1, paper_info.points);
    }
    printer(paper_info.points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 706);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(
            solve_part2(),
            "#....###..####...##.###....##.####.#..#\n\
             #....#..#.#.......#.#..#....#.#....#..#\n\
             #....#..#.###.....#.###.....#.###..####\n\
             #....###..#.......#.#..#....#.#....#..#\n\
             #....#.#..#....#..#.#..#.#..#.#....#..#\n\
             ####.#..#.#.....##..###...##..####.#..#\n\
             "
        );
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
