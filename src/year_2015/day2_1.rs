// Day 2: I Was Told There Would Be No Math
// Problem 1

pub fn solve(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for word in line.split_whitespace() {
            let num = word.parse::<i32>().unwrap();
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }
        sum += max - min;
    }
    sum
}
