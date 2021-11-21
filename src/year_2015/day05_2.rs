// Day 5: Doesn't He Have Intern-Elves For This?
// Problem 2

// Regex time!
use fancy_regex::Regex;

pub fn solve(input: &str) -> usize {
    let mut nice_strings = 0;
    for word in input.split_whitespace() {
        nice_strings += if is_nice_string(word) { 1 } else { 0 };
    }
    nice_strings
}

pub fn is_nice_string(word: &str) -> bool {
    let repeat_letter = Regex::new(r"([a-z][a-z]).*\1").unwrap();
    let pair = Regex::new(r"([a-z]).\1").unwrap();
    if repeat_letter
        .is_match(word)
        .expect("Something went wrong trying to match regex")
        && pair
            .is_match(word)
            .expect("Something want wrong trying to match regex")
    {
        return true;
    }
    false
}
