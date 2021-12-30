use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("File should exist.");
    let lines: Vec<&str> = input.split("\n").collect();
    let split_lines: Vec<(&str, &str)> =
        lines.iter().map(|&s| s.split_once(" ").unwrap()).collect();
    let coordinates = split_lines
        .iter()
        .fold((0, 0), |(horizontal, depth), (command, value)| {
            match (*command, value.parse::<i32>().unwrap()) {
                // Trying to use i16 here results in overflow for the final multiplication
                ("forward", value) => (horizontal + value, depth),
                ("down", value) => (horizontal, depth + value),
                ("up", value) => (horizontal, depth - value),
                _ => unreachable!(),
            }
        });
    let (horizontal, depth) = coordinates;
    println!("{}", horizontal * depth); // 2073315
}
