mod year_2015;
mod read_input;

fn main() {
    // print print_hello from year_2015
    let input: String = read_input::read_file_to_string("inputs/2015/day5.txt");
    year_2015::print_answer(5, 1, input);
}
