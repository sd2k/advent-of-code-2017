extern crate lib;
use lib::get_file_to_string;

const FILENAME: &'static str = "../input";

fn convert_line(input: &str) -> Vec<usize> {
    input.split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn process_line_min_max(line: &[usize]) -> usize {
    line.iter().max().unwrap() - line.iter().min().unwrap()
}

fn process_line_divisors(line: &[usize]) -> usize {
    let mut line_desc: Vec<usize> = line.to_vec();
    line_desc.sort_by(|a, b| b.cmp(a));
    let line_asc = line_desc.iter().rev().cloned().collect::<Vec<usize>>();
    for high in line_desc.iter() {
        for low in line_asc.iter() {
            if high % low == 0 && high != low {
                return high / low
            }
        }
    }
    // We've been assured that there'll always be a pair of numbers that
    // divide exactly
    unreachable!()
}

fn main() {
    let i = get_file_to_string(FILENAME).unwrap();
    println!("{}", i);
    let lines = i.lines()
        .map(convert_line)
        .collect::<Vec<Vec<usize>>>();

    let part1: usize = lines.iter()
        .map(|l| process_line_min_max(&l))
        .sum();
    println!("{}", part1);

    let part2: usize = lines.iter()
        .map(|l| process_line_divisors(&l))
        .sum();
    println!("{}", part2);
}
