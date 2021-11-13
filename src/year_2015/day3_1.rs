// Day 3: Perfectly Spherical Houses in a Vacuum
// Problem 1

// Import hashmap for coordinates
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    // HashMap which contains coordinates and if they've been visited
    let mut visited_houses: HashMap<(i32, i32), bool> = HashMap::new();

    let (mut x, mut y) = (0, 0);

    visited_houses.insert((x, y), true); // Starting position

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => break // Assume anything else is EOF
        }

        visited_houses.insert((x, y), true);
    }
    visited_houses.len()
}
