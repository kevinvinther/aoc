// Day 5: Doesn't He Have Intern-Elves For This?
// Problem 1

// TODO: this is a very slow solution, but it works
pub fn solve(input: &str) -> usize {
    let vowels: &str = "aeiou";
    let double_letters = vec!["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"];
    let not_allowed = vec!["ab", "cd", "pq", "xy"];
    let mut count: usize = 0;
    let mut nice_meter;
    let mut vowel_count;

    // TODO: I'm sure this can be done more efficiently
    for word in input.split_whitespace() {
        nice_meter = 0;
        vowel_count = 0;
        for letter in word.chars() {
            if vowels.contains(letter) {
                vowel_count += 1;
            }
        }
        'a: for double_letter in double_letters.iter() {
            if word.contains(double_letter) {
                nice_meter += 1;
                break 'a;
            }
        }
        if vowel_count >= 3 {
            nice_meter += 1;
        }

        for pair in &not_allowed {
            if word.contains(pair) {
                nice_meter = 0;
                break;
            }
        }
        if nice_meter >= 2 {
            count += 1;
        }
    }
    count
}
