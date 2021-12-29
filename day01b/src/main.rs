use std::fs;

fn main() {
    let input: String = fs::read_to_string("../day01a/input.txt").expect("File should exist.");
    let numbers: Vec<u16> = input.split("\n").map(
        |s| s.to_string().parse::<u16>().expect("Should be integer")).collect();
    let count_increasing: &usize = &numbers.windows(4).filter(|&w| w[3] > w[0]).count();
    println!("{}", *count_increasing);  // 1518
}