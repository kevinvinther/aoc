// Function to read input from file
// Language: rust
// Path: src/year_2015/read_input.rs
// Function to read input from file
use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents.parse().expect("Could not parse input")
}

