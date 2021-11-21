// Day 4: The Ideal Stocking Stuffer
// Problem 2
use md5;

// TODO: Implement own solution without using the md5 crate
pub fn solve(input: &str) -> u32 {
    let mut count = 0; 
    loop {
        let hasher_input = format!("{}{}", input.to_owned().trim(), count);

        let result = md5::compute(hasher_input);

        let result = format!("{:?}", result);

        if result.starts_with("000000") {
            println!("{}", result);
            return count; 
        }

        count += 1; 
    }
}
