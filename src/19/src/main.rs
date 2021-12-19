use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, Sub};
use util::input::get_input_string;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn rotate(&self, id: i64) -> Point {
        match id {
            0 => Point {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            1 => Point {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            2 => Point {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            3 => Point {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            4 => Point {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            5 => Point {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            6 => Point {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            7 => Point {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            8 => Point {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            9 => Point {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            10 => Point {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },
            11 => Point {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            12 => Point {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            13 => Point {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            14 => Point {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            15 => Point {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            16 => Point {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            17 => Point {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            18 => Point {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            19 => Point {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            20 => Point {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            21 => Point {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            22 => Point {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            23 => Point {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            _ => panic!("Something went wrong"),
        }
    }
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn get_input() -> Vec<Vec<Point>> {
    let strings = get_input_string("19");
    let mut given_scanners = Vec::new();
    {
        let mut this_scanner = Vec::new();
        for string in strings {
            if string.len() == 0 {
                given_scanners.push(this_scanner);
                this_scanner = Vec::new();
            } else {
                match &string[..3] {
                    "---" => {
                        continue;
                    }
                    _ => {
                        let (one, mid) = string.split_once(',').unwrap();
                        let (two, las) = mid.split_once(',').unwrap();
                        this_scanner.push(Point {
                            x: one.parse::<i64>().unwrap(),
                            y: two.parse::<i64>().unwrap(),
                            z: las.parse::<i64>().unwrap(),
                        });
                    }
                }
            }
        }
        given_scanners.push(this_scanner);
    }
    given_scanners
}

fn merge_scan(known_scanners: &mut HashSet<Point>, current_scanner: &Vec<Point>) -> Option<Point> {
    for id in 0..24 {
        let all_rotations = current_scanner
            .iter()
            .map(|&v| v.rotate(id))
            .collect::<Vec<_>>();
        let all_offsets: Vec<Point> = known_scanners
            .iter()
            .map(|&item_x| all_rotations.iter().map(move |&item_y| (item_x, item_y)))
            .flatten()
            .map(|(p1, p2)| p1 - p2)
            .collect();
        for offset in all_offsets {
            let translated = all_rotations.iter().map(|p| *p + offset);
            if translated
                .clone()
                .filter(|v| known_scanners.contains(v))
                .count()
                >= 12
            {
                known_scanners.extend(translated);
                return Some(offset);
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let mut given_scanners = get_input();
    let mut known_scanners = given_scanners.remove(0).into_iter().collect::<HashSet<_>>();
    loop {
        for i in (0..given_scanners.len()).rev() {
            if let Some(_) = merge_scan(&mut known_scanners, &given_scanners[i]) {
                given_scanners.remove(i);
            }
        }
        if given_scanners.is_empty() {
            break;
        }
    }
    known_scanners.len().try_into().unwrap()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let mut given_scanners = get_input();
    let mut known_scanners = given_scanners.remove(0).into_iter().collect::<HashSet<_>>();
    let mut offsets = Vec::from([Point { x: 0, y: 0, z: 0 }]);
    loop {
        for i in (0..given_scanners.len()).rev() {
            if let Some(offset) = merge_scan(&mut known_scanners, &given_scanners[i]) {
                offsets.push(offset);
                given_scanners.remove(i);
            }
        }
        if given_scanners.is_empty() {
            break;
        }
    }
    offsets
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            offsets[i + 1..].iter().map(move |b| {
                let dp = *a - *b;
                (dp.x).abs() + (dp.y).abs() + (dp.z).abs()
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 512);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 16802);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
