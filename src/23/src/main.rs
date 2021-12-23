use std::collections::BinaryHeap;
use std::collections::HashSet;
use util::input::get_input_string;

const ROOM_SLOT: &'static [i64] = &[2, 4, 6, 8];
const HALL_SLOT: &'static [i64] = &[0, 1, 3, 5, 7, 9, 10];
const COST_TABLE: &'static [i64] = &[1, 10, 100, 1000];

fn has_reached(a: &Vec<(char, char)>, b: &Vec<(char, char)>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i].0 != b[i].0 || a[i].1 != b[i].1 {
            return false;
        }
    }
    true
}

fn search(rooms: Vec<(char, char)>) -> i64 {
    let dest: Vec<(char, char)> = Vec::from([('A', 'A'), ('B', 'B'), ('C', 'C'), ('D', 'D')]);
    let mut cost_heap = BinaryHeap::new();
    let mut vis: HashSet<(Vec<(char, char)>, Vec<char>)> = HashSet::new();
    cost_heap.push((
        0,
        (
            rooms.clone(),
            Vec::from([' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']),
        ),
    ));
    while !cost_heap.is_empty() {
        let (cost, (current_state_room, current_state_hallway)) = cost_heap.pop().unwrap();
        if has_reached(&current_state_room, &dest) {
            return -cost;
        }
        if !vis.insert((current_state_room.to_vec(), current_state_hallway.to_vec())) {
            continue;
        }
        for i in 0..4 {
            if current_state_room[i].0 == dest[i].0 && current_state_room[i].1 == dest[i].1 {
                continue;
            }
            let to_move;
            let to_move_row;
            let to_move_id;
            if current_state_room[i].0 != ' ' {
                to_move_row = 0;
                to_move = current_state_room[i].0;
            } else if current_state_room[i].1 != ' ' {
                to_move_row = 1;
                to_move = current_state_room[i].1;
            } else {
                continue;
            }
            to_move_id = match to_move {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                _ => panic!("Something went wrong"),
            };
            for h in HALL_SLOT {
                let hsize = *h as usize;
                let left_bound = hsize.min(2 * (i + 1));
                let right_bound = hsize.max(2 * (i + 1)) + 1;
                let bound = &current_state_hallway[left_bound..right_bound];
                if bound.iter().all(|&x| x == ' ') {
                    let new_cost = -cost
                        + COST_TABLE[to_move_id]
                            * (to_move_row + 1 + (2 * (i + 1) as i64 - h).abs());
                    let mut new_state_room = current_state_room.to_vec();
                    match to_move_row {
                        0 => {
                            new_state_room[i].0 = ' ';
                        }
                        _ => {
                            new_state_room[i].1 = ' ';
                        }
                    }
                    let mut new_state_hallway = current_state_hallway.to_vec();
                    new_state_hallway[*h as usize] = to_move;
                    cost_heap.push((-new_cost, (new_state_room, new_state_hallway)));
                }
            }
        }
        for h in HALL_SLOT {
            let hall_loc = *h as usize;
            let item = current_state_hallway[hall_loc];
            let room_loc;
            let i;
            if item == ' ' {
                continue;
            } else {
                i = match item {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => panic!("Something went wrong"),
                };
                room_loc = ROOM_SLOT[i] as usize;
            }
            let hrizontal = match hall_loc > room_loc {
                true => &current_state_hallway[room_loc..hall_loc],
                _ => &current_state_hallway[hall_loc + 1..room_loc + 1],
            };
            if hrizontal.iter().all(|&x| x == ' ') {
                if (current_state_room[i].0 == ' ' || current_state_room[i].0 == item)
                    && (current_state_room[i].1 == ' ' || current_state_room[i].1 == item)
                {
                    let mut vertical = 0;
                    if current_state_room[i].0 == ' ' && current_state_room[i].1 == ' ' {
                        vertical += 1;
                    }
                    let new_cost = -cost
                        + COST_TABLE[i]
                            * (vertical as i64 + 1 + (room_loc as i64 - *h).abs()) as i64;

                    let mut new_state_room = current_state_room.to_vec();
                    match vertical {
                        0 => {
                            new_state_room[i].0 = item;
                        }
                        _ => {
                            new_state_room[i].1 = item;
                        }
                    }
                    let mut new_state_hallway = current_state_hallway.to_vec();
                    new_state_hallway[hall_loc] = ' ';
                    cost_heap.push((-new_cost, (new_state_room, new_state_hallway)));
                }
            }
        }
    }
    unreachable!()
}

fn has_reached2(a: &Vec<(char, char, char, char)>, b: &Vec<(char, char, char, char)>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i].0 != b[i].0 || a[i].1 != b[i].1 || a[i].2 != b[i].2 || a[i].3 != b[i].3 {
            return false;
        }
    }
    true
}

fn search2(rooms: Vec<(char, char, char, char)>) -> i64 {
    let dest: Vec<(char, char, char, char)> = Vec::from([
        ('A', 'A', 'A', 'A'),
        ('B', 'B', 'B', 'B'),
        ('C', 'C', 'C', 'C'),
        ('D', 'D', 'D', 'D'),
    ]);
    let mut cost_heap = BinaryHeap::new();
    let mut vis: HashSet<(Vec<(char, char, char, char)>, Vec<char>)> = HashSet::new();
    cost_heap.push((
        0,
        (
            rooms.clone(),
            Vec::from([' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']),
        ),
    ));
    while !cost_heap.is_empty() {
        let (cost, (current_state_room, current_state_hallway)) = cost_heap.pop().unwrap();
        if has_reached2(&current_state_room, &dest) {
            return -cost;
        }
        if !vis.insert((current_state_room.to_vec(), current_state_hallway.to_vec())) {
            continue;
        }
        for i in 0..4 {
            if current_state_room[i].0 == dest[i].0
                && current_state_room[i].1 == dest[i].1
                && current_state_room[i].2 == dest[i].2
                && current_state_room[i].3 == dest[i].3
            {
                continue;
            }
            let to_move;
            let to_move_row;
            let to_move_id;
            if current_state_room[i].0 != ' ' {
                to_move_row = 0;
                to_move = current_state_room[i].0;
            } else if current_state_room[i].1 != ' ' {
                to_move_row = 1;
                to_move = current_state_room[i].1;
            } else if current_state_room[i].2 != ' ' {
                to_move_row = 2;
                to_move = current_state_room[i].2;
            } else if current_state_room[i].3 != ' ' {
                to_move_row = 3;
                to_move = current_state_room[i].3;
            } else {
                continue;
            }
            to_move_id = match to_move {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                _ => panic!("Something went wrong"),
            };
            for h in HALL_SLOT {
                let hsize = *h as usize;
                let left_bound = hsize.min(2 * (i + 1));
                let right_bound = hsize.max(2 * (i + 1)) + 1;
                let bound = &current_state_hallway[left_bound..right_bound];
                if bound.iter().all(|&x| x == ' ') {
                    let new_cost = -cost
                        + COST_TABLE[to_move_id]
                            * (to_move_row + 1 + (2 * (i + 1) as i64 - h).abs());
                    let mut new_state_room = current_state_room.to_vec();
                    match to_move_row {
                        0 => {
                            new_state_room[i].0 = ' ';
                        }
                        1 => {
                            new_state_room[i].1 = ' ';
                        }
                        2 => {
                            new_state_room[i].2 = ' ';
                        }
                        _ => {
                            new_state_room[i].3 = ' ';
                        }
                    }
                    let mut new_state_hallway = current_state_hallway.to_vec();
                    new_state_hallway[*h as usize] = to_move;
                    cost_heap.push((-new_cost, (new_state_room, new_state_hallway)));
                }
            }
        }
        for h in HALL_SLOT {
            let hall_loc = *h as usize;
            let item = current_state_hallway[hall_loc];
            let room_loc;
            let i;
            if item == ' ' {
                continue;
            } else {
                i = match item {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => panic!("Something went wrong"),
                };
                room_loc = ROOM_SLOT[i] as usize;
            }
            let hrizontal = match hall_loc > room_loc {
                true => &current_state_hallway[room_loc..hall_loc],
                _ => &current_state_hallway[hall_loc + 1..room_loc + 1],
            };
            if hrizontal.iter().all(|&x| x == ' ') {
                if (current_state_room[i].0 == ' ' || current_state_room[i].0 == item)
                    && (current_state_room[i].1 == ' ' || current_state_room[i].1 == item)
                    && (current_state_room[i].2 == ' ' || current_state_room[i].2 == item)
                    && (current_state_room[i].3 == ' ' || current_state_room[i].3 == item)
                {
                    let mut vertical = 0;
                    if current_state_room[i].0 == ' ' && current_state_room[i].1 == ' ' {
                        vertical += 1;
                    }
                    if current_state_room[i].1 == ' ' && current_state_room[i].2 == ' ' {
                        vertical += 1;
                    }
                    if current_state_room[i].2 == ' ' && current_state_room[i].3 == ' ' {
                        vertical += 1;
                    }
                    let new_cost = -cost
                        + COST_TABLE[i]
                            * (vertical as i64 + 1 + (room_loc as i64 - *h).abs()) as i64;

                    let mut new_state_room = current_state_room.to_vec();
                    match vertical {
                        0 => {
                            new_state_room[i].0 = item;
                        }
                        1 => {
                            new_state_room[i].1 = item;
                        }
                        2 => {
                            new_state_room[i].2 = item;
                        }
                        _ => {
                            new_state_room[i].3 = item;
                        }
                    }
                    let mut new_state_hallway = current_state_hallway.to_vec();
                    new_state_hallway[hall_loc] = ' ';
                    cost_heap.push((-new_cost, (new_state_room, new_state_hallway)));
                }
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("23");
    let first = &strings[2].chars().collect::<Vec<_>>();
    let second = &strings[3].chars().collect::<Vec<_>>();
    let mut rooms: Vec<(char, char)> = Vec::new();
    for i in (3..10).step_by(2) {
        rooms.push((first[i], second[i]));
    }
    assert_eq!(rooms.len(), 4);
    search(rooms)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("23");
    let first = &strings[2].chars().collect::<Vec<_>>();
    let second = &strings[3].chars().collect::<Vec<_>>();
    let mut rooms: Vec<(char, char, char, char)> = Vec::new();
    for i in (3..10).step_by(2) {
        match rooms.len() {
            0 => {
                rooms.push((first[i], 'D', 'D', second[i]));
            }
            1 => {
                rooms.push((first[i], 'C', 'B', second[i]));
            }
            2 => {
                rooms.push((first[i], 'B', 'A', second[i]));
            }
            _ => {
                rooms.push((first[i], 'A', 'C', second[i]));
            }
        }
    }
    assert_eq!(rooms.len(), 4);
    search2(rooms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_reached() {
        let v1 = Vec::from([('B', 'A'), ('C', 'D')]);
        let v2 = Vec::from([('B', 'A'), ('C', 'D')]);
        assert_eq!(has_reached(&v1, &v2), true);
    }

    #[test]
    fn test_has_not_reached() {
        let v1 = Vec::from([('B', 'A'), ('C', 'D')]);
        let v2 = Vec::from([('A', 'A'), ('D', 'D')]);
        assert_eq!(has_reached(&v1, &v2), false);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 15385);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 49803);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
