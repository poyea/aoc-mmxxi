use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;

pub fn get_input_integer(id: &str) -> Vec<i64> {
    // read from file at "src/<id>/src/input.txt"
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
    let reader = BufReader::new(file);
    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    numbers
}

pub fn get_input_string(id: &str) -> Vec<String> {
    // read from file at "src/<id>/src/input.txt"
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
    let reader = BufReader::new(file);
    let strings: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse string from file"))
        .collect();
    strings
}
