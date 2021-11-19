// Day 5: Doesn't He Have Intern-Elves For This?
// Problem 1

// TODO: this is a very slow solution, but it works
pub fn solve(input: &str) -> usize {
    let vowels = vec!["a", "e", "i", "o", "u"];
    let double_letters = vec!["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"];
    let not_allowed = vec!["ab", "cd", "pq", "xy"];
    let mut count = 0;
    let mut nice_meter; 
    let mut vowel_count;
    for word in input.split_whitespace() {
        nice_meter = 0; 
        vowel_count = 0;
        'a: for letter in &vowels {
            if word.contains(letter) {
                vowel_count += 1;
                if vowel_count >= 3 {
                    nice_meter += 1;
                    break 'a;
                }
            }
        }
        'b: for letter in &double_letters {
            if word.contains(letter) {
                nice_meter += 1; 
                break 'b;
            }
        }
        'c: for letter in &not_allowed {
            if word.contains(letter) {
                nice_meter = 0; 
                break 'c;
            }
        }
        if nice_meter >= 2 {
            count += 1; 
        }
    }
    count
}
