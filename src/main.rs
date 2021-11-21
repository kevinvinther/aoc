mod year_2015;
mod read_input;

fn main() {
    let input: String = read_input::read_file_to_string("inputs/2015/day6.txt");
    year_2015::print_answer(6, 1, input);
}
