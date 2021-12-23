use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;

fn get_reader(id: &str) -> BufReader<File> {
    let root_loc = "src/".to_owned() + id + "/src/input.txt";
    let lib_loc = "src/input.txt";
    let pwd_loc = "input.txt";
    let file = match File::open(root_loc) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::open(lib_loc) {
                Ok(fc) => fc,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => match File::open(pwd_loc) {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem opening the file: {:?}", e),
                    },
                    other_error => panic!("Problem opening the file: {:?}", other_error),
                },
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    BufReader::new(file)
}

/// Get a `Vec<i64>` given the day ID
pub fn get_input_integer(id: &str) -> Vec<i64> {
    get_reader(id)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

/// Get a `Vec<String>` given the day ID
pub fn get_input_string(id: &str) -> Vec<String> {
    get_reader(id)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

/// Split a string and parse as integers
pub fn split_string(delimiter: &str, string: &str) -> Vec<i64> {
    string
        .split(delimiter)
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub type Grid = Vec<Vec<i64>>;
pub type GridCell = (usize, usize);

pub fn make_grid(strings: &Vec<String>) -> Grid {
    let mut grid: Grid = Vec::new();
    for line in strings {
        let mut line_vec = Vec::new();
        for c in line.chars() {
            line_vec.push(c.to_digit(10).unwrap() as i64);
        }
        grid.push(line_vec);
    }
    grid
}
