pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;
pub mod day3_1;
pub mod day3_2;
pub mod day4_1;
pub mod day4_2;

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
        _ => println!("Day not yet implemented!"),
    }
}
