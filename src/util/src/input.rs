use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;

fn get_reader(id: &str) -> BufReader<File> {
    let root_loc = "src/".to_owned() + id + "/src/input.txt";
    let lib_loc = "src/input.txt";
    let file = File::open(root_loc);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::open(lib_loc) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    BufReader::new(file)
}

/// Get a `Vec<i64>` given the day ID
pub fn get_input_integer(id: &str) -> Vec<i64> {
    let numbers: Vec<i64> = get_reader(id)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    numbers
}

/// Get a `Vec<String>` given the day ID
pub fn get_input_string(id: &str) -> Vec<String> {
    let strings: Vec<String> = get_reader(id)
        .lines()
        .map(|l| l.expect("Could not parse string from file"))
        .collect();
    strings
}

/// Split a string and parse as integers
pub fn split_string(delimiter: &str, string: &str) -> Vec<i64> {
    string
        .split(delimiter)
        .map(|x| x.parse().unwrap())
        .collect()
}
