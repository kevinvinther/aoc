// Day 1: Not Quite Lisp
// Problem 1
// Problem description: https://adventofcode.com/2015/day/1/

pub fn solve(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1
        }
        else if c == ')' {
            floor -= 1
        }
    }
    floor
}

