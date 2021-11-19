pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;
pub mod day3_1;
pub mod day3_2;
pub mod day4_1;
pub mod day4_2;
pub mod day5_1;
// pub mod day5_2;
// pub mod day6_1;
// pub mod day6_2;
// pub mod day7_1;
// pub mod day7_2;
// pub mod day8_1;
// pub mod day8_2;
// pub mod day9_1;
// pub mod day9_2;
// pub mod day10_1;
// pub mod day10_2;
// pub mod day11_1;
// pub mod day11_2;
// pub mod day12_1;
// pub mod day12_2;
// pub mod day13_1;
// pub mod day13_2;
// pub mod day14_1;
// pub mod day14_2;
// pub mod day15_1;
// pub mod day15_2;
// pub mod day16_1;
// pub mod day16_2;
// pub mod day17_1;
// pub mod day17_2;
// pub mod day18_1;
// pub mod day18_2;
// pub mod day19_1;
// pub mod day19_2;
// pub mod day20_1;
// pub mod day20_2;
// pub mod day21_1;
// pub mod day21_2;
// pub mod day22_1;
// pub mod day22_2;
// pub mod day23_1;
// pub mod day23_2;
// pub mod day24_1;
// pub mod day24_2;
// pub mod day25_1;
// pub mod day25_2;

pub fn print_answer(day: u32, problem: u32, input: String) {
    match (day, problem) {
        (1, 1) => println!("{}", day1_1::solve(&input)),
        (1, 2) => println!("{}", day1_2::solve(&input)),
        (2, 1) => println!("{}", day2_1::solve(&input)),
        (2, 2) => println!("{}", day2_2::solve(&input)),
        (3, 1) => println!("{}", day3_1::solve(&input)),
        (3, 2) => println!("{}", day3_2::solve(&input)),
        (4, 1) => println!("{}", day4_1::solve(&input)),
        (4, 2) => println!("{}", day4_2::solve(&input)),
        (5, 1) => println!("{}", day5_1::solve(&input)),
        // (5, 2) => println!("{}", day5_2::solve(&input)),
        // (6, 1) => println!("{}", day6_1::solve(&input)),
        // (6, 2) => println!("{}", day6_2::solve(&input)),
        // (7, 1) => println!("{}", day7_1::solve(&input)),
        // (7, 2) => println!("{}", day7_2::solve(&input)),
        // (8, 1) => println!("{}", day8_1::solve(&input)),
        // (8, 2) => println!("{}", day8_2::solve(&input)),
        // (9, 1) => println!("{}", day9_1::solve(&input)),
        // (9, 2) => println!("{}", day9_2::solve(&input)),
        // (10, 1) => println!("{}", day10_1::solve(&input)),
        // (10, 2) => println!("{}", day10_2::solve(&input)),
        // (11, 1) => println!("{}", day11_1::solve(&input)),
        // (11, 2) => println!("{}", day11_2::solve(&input)),
        // (12, 1) => println!("{}", day12_1::solve(&input)),
        // (12, 2) => println!("{}", day12_2::solve(&input)),
        // (13, 1) => println!("{}", day13_1::solve(&input)),
        // (13, 2) => println!("{}", day13_2::solve(&input)),
        // (14, 1) => println!("{}", day14_1::solve(&input)),
        // (14, 2) => println!("{}", day14_2::solve(&input)),
        // (15, 1) => println!("{}", day15_1::solve(&input)),
        // (15, 2) => println!("{}", day15_2::solve(&input)),
        // (16, 1) => println!("{}", day16_1::solve(&input)),
        // (16, 2) => println!("{}", day16_2::solve(&input)),
        // (17, 1) => println!("{}", day17_1::solve(&input)),
        // (17, 2) => println!("{}", day17_2::solve(&input)),
        // (18, 1) => println!("{}", day18_1::solve(&input)),
        // (18, 2) => println!("{}", day18_2::solve(&input)),
        // (19, 1) => println!("{}", day19_1::solve(&input)),
        // (19, 2) => println!("{}", day19_2::solve(&input)),
        // (20, 1) => println!("{}", day20_1::solve(&input)),
        // (20, 2) => println!("{}", day20_2::solve(&input)),
        // (21, 1) => println!("{}", day21_1::solve(&input)),
        // (21, 2) => println!("{}", day21_2::solve(&input)),
        // (22, 1) => println!("{}", day22_1::solve(&input)),
        // (22, 2) => println!("{}", day22_2::solve(&input)),
        // (23, 1) => println!("{}", day23_1::solve(&input)),
        // (23, 2) => println!("{}", day23_2::solve(&input)),
        // (24, 1) => println!("{}", day24_1::solve(&input)),
        // (24, 2) => println!("{}", day24_2::solve(&input)),
        // (25, 1) => println!("{}", day25_1::solve(&input)),
        // (25, 2) => println!("{}", day25_2::solve(&input)),
        (_, _) => println!("Day {}, problem {} not yet implemented", day, problem)
    }
}
