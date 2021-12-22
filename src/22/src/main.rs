use std::collections::HashMap;
use util::input::{get_input_string, split_string};

type Coords = (i64, i64);

#[allow(dead_code)]
fn parse_string(string: &String) -> (&str, Vec<i64>, Vec<i64>, Vec<i64>) {
    let (op, coords) = string.split_once(" ").unwrap();
    let (xco, rest) = coords.split_once(",").unwrap();
    let (yco, zco) = rest.split_once(",").unwrap();
    let (_, xrange) = xco.split_once("=").unwrap();
    let (_, yrange) = yco.split_once("=").unwrap();
    let (_, zrange) = zco.split_once("=").unwrap();
    let x = split_string("..", xrange);
    let y = split_string("..", yrange);
    let z = split_string("..", zrange);
    (op, x, y, z)
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("22");
    let mut map: HashMap<(i64, i64, i64), i64> = HashMap::new();
    for string in strings {
        let (op, x, y, z) = parse_string(&string);
        if x[0].abs() > 50
            || x[1].abs() > 50
            || y[0].abs() > 50
            || y[1].abs() > 50
            || z[0].abs() > 50
            || z[1].abs() > 50
        {
            break;
        }
        match op {
            "on" => {
                for i in x[0]..=x[1] {
                    for j in y[0]..=y[1] {
                        for k in z[0]..=z[1] {
                            *map.entry((i, j, k)).or_insert(1) = 1;
                        }
                    }
                }
            }
            _ => {
                for i in x[0]..=x[1] {
                    for j in y[0]..=y[1] {
                        for k in z[0]..=z[1] {
                            *map.entry((i, j, k)).or_insert(0) = 0;
                        }
                    }
                }
            }
        };
    }
    map.values().sum::<i64>()
}

#[allow(dead_code)]
fn find_inside_cubes(cube: Vec<Coords>, join: &Vec<Coords>) -> Option<Vec<Vec<Coords>>> {
    let mut p = Vec::new();
    for i in 0..3 {
        if cube[i].1 < join[i].0 || join[i].1 < cube[i].0 {
            return Some(Vec::from([cube]));
        }
        p.push((cube[i].0.max(join[i].0), cube[i].1.min(join[i].1)));
    }
    let mut possible = Vec::from([
        Vec::from([(cube[0].0, p[0].0 - 1), cube[1], p[2]]),
        Vec::from([(p[0].1 + 1, cube[0].1), cube[1], p[2]]),
        Vec::from([p[0], (cube[1].0, p[1].0 - 1), p[2]]),
        Vec::from([p[0], (p[1].1 + 1, cube[1].1), p[2]]),
        Vec::from([cube[0], cube[1], (cube[2].0, p[2].0 - 1)]),
        Vec::from([cube[0], cube[1], (p[2].1 + 1, cube[2].1)]),
    ]);
    possible.retain(|g| {
        let x = g[0];
        let y = g[1];
        let z = g[2];
        x.0 <= x.1 && y.0 <= y.1 && z.0 <= z.1
    });
    Some(possible)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("22");
    let mut cubes: Vec<Vec<Coords>> = Vec::new();
    for string in strings {
        let (op, x, y, z) = parse_string(&string);
        let block = Vec::from([(x[0], x[1]), (y[0], y[1]), (z[0], z[1])]);
        let mut cur_cubes: Vec<Vec<Coords>> = Vec::new();
        for vertex in cubes {
            if let Some(e) = find_inside_cubes(vertex, &block) {
                for vec in e {
                    cur_cubes.push(vec);
                }
            }
        }
        if op == "on" {
            cur_cubes.push(block);
        }
        cubes = cur_cubes;
    }
    let mut sum = 0;
    for c in cubes {
        let mut v = 1;
        for idx in c {
            v *= idx.1 - idx.0 + 1;
        }
        sum += v;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 620241);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1284561759639324);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
