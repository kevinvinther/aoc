// Day 6: Probably a Fire Hazard
// Problem 1

use std::collections::HashMap;

// TODO: This would be a fun one to visualize
pub fn solve(input: &str) -> usize {
    let input = input
        .replace("turn on", "0")
        .replace("turn off", "1")
        .replace("toggle", "2");

    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();

    for line in input.lines() {
        let command = line.chars().nth(0).unwrap();
        let mut coords = line.split(",");
        // HACK: there's gotta be a better way to do this

        let mut x_1 = coords.next().unwrap().split(" ");
        x_1.next();
        let x_1 = x_1.next().unwrap().parse::<i32>().unwrap();

        let mut temp = coords.next().unwrap().split(" ");

        let y_1 = temp.next().unwrap().parse::<i32>().unwrap();

        temp.next();

        let x_2 = temp.next().unwrap().parse::<i32>().unwrap();

        let y_2 = coords.next().unwrap().parse::<i32>().unwrap();

       
        if command == '0' {
            for  x in x_1..=x_2 {
                for y in y_1..=y_2 {
                    grid.insert((x, y), true);
                }
            }
        } else if command == '1' {
            for x in x_1..=x_2 {
                for y in y_1..=y_2 {
                    grid.insert((x, y), false);
                }
            }
        } else if command == '2' {
            for x in x_1..=x_2 {
                for y in y_1..=y_2 {
                    grid.insert((x, y), !grid.get(&(x, y)).unwrap_or(&false));
                }
            }
        } 
    }
    let mut light_on = 0; 
    for val in grid.values() {
        if *val {
            light_on += 1;
        }
    }
    light_on
}
