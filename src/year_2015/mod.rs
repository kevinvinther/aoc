pub mod read_input;
pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;

pub fn print_answer(day: u32, problem: u32, path: &str) {
    // Read text from inputs/2015/day1_1.txt
    let input = read_input::read_file_to_string(&path);
    match (day, problem) {
        (1, 1) => println!("{}", day1_1::solve(&input)),
        (1, 2) => println!("{}", day1_2::solve(&input)),
        (2, 1) => println!("{}", day2_1::solve(&input)),
        (2, 2) => println!("{}", day2_2::solve(&input)),
        _ => println!("Day not yet implemented!"),
    }
}
