// Day 2: I Was Told There Would Be No Math
// Problem 1

pub fn solve(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let split_vec = input
        .split('\n')
        .collect::<Vec<&str>>();
    for s in split_vec {
        let s = s.split('x')
            .collect::<Vec<&str>>();

        // FIX: There definitely is a better way to do this
        if s[0] == "" {
            break;
        }

        let length: i32 = s[0].parse::<i32>().unwrap();
        let width: i32 = s[1].parse::<i32>().unwrap();
        let height: i32 = s[2].parse::<i32>().unwrap();

        // Make a vector of the dimensions
        let sides = vec![length*width, width*height, height*length];

        // Find minimum side
        let min_value = sides.iter().min().expect("Could not find a minimum value");

        // Sum the dimensions
        sum += 2 * length * width + 2 * width * height + 2 * height * length + min_value;
    }
    sum
}
