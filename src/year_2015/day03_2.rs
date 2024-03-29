// Day 3: Perfectly Spherical Houses in a Vacuum
// Problem 2

// Import hashmap for coordinates
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    // HashMap which contains coordinates and if they've been visited
    let mut visited_houses: HashMap<(i32, i32), bool> = HashMap::new();

    let (mut santa_x, mut santa_y, mut robo_x, mut robo_y) = (0, 0, 0, 0);

    let mut count = 0; 

    visited_houses.insert((santa_x, santa_y), true); // Starting position

    for c in input.chars() {
        count += 1; 
        if count % 2 == 0 {
            match c {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => break // Assume anything else is EOF
            }
            visited_houses.insert((santa_x, santa_y), true);
        } else {
            match c {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                _ => break // Assume anything else is EOF
            }
            visited_houses.insert((robo_x, robo_y), true);
        }
    }
    visited_houses.len() // No need for +1 

}
