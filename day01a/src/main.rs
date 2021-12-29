use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("File should exist.");
    let numbers: Vec<u16> = input.split("\n").map(
        |s| s.to_string().parse::<u16>().expect("Should be integer")).collect();
    let count_increasing: &usize = &numbers.windows(2).filter(|&w| w[1] > w[0]).count();
    println!("{}", *count_increasing);  // 1482
}
