extern crate lib;
use lib::get_file_to_string;

const FILENAME: &'static str = "../input";


fn sum_shifted_adjacent_numbers(input: &str, shift: usize) -> u32 {
    let input_shifted = input.chars().cycle().skip(shift);
    input
        .chars()
        .zip(input_shifted)
        .take(input.len())
        .filter(|&a| a.0 == a.1)
        .map(|(a, _)| a.to_digit(10).unwrap())
        .sum()
}


fn main() {
    let input = get_file_to_string(FILENAME).unwrap();
    println!("{}", input);
    let sum = sum_shifted_adjacent_numbers(&input, 1);
    println!("{}", sum);
    let sum2 = sum_shifted_adjacent_numbers(&input, input.len()/2);
    println!("{}", sum2);
}
