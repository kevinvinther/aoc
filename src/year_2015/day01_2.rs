// Day 1: Not Quite Lisp
// Problem 2

pub fn solve(input: &str) -> u32 {
    let mut floor: i32 = 0;
    let mut position: u32 = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1
        }
        else if c == ')' {
            floor -= 1
        }
        position += 1; 
        if floor == -1 {
            return position;
        }
    }
    eprintln!("Could not find floor -1");
    1
}
