use std::fs;

fn main() {
    let input: String = fs::read_to_string("../day02a/input.txt").expect("File should exist.");
    let lines: Vec<&str> = input.split("\n").collect();
    let split_lines: Vec<(&str, &str)> =
        lines.iter().map(|&s| s.split_once(" ").unwrap()).collect();
    let coordinates = split_lines
        .iter()
        .fold((0, 0, 0), |(horizontal, depth, aim), (command, value)| {
            match (*command, value.parse::<i32>().unwrap()) {
                // Trying to use i16 here results in overflow for the final multiplication
                ("forward", value) => (horizontal + value, depth + aim * value, aim),
                ("down", value) => (horizontal, depth, aim + value),
                ("up", value) => (horizontal, depth, aim - value),
                _ => unreachable!(),
            }
        });
    let (horizontal, depth, _) = coordinates;
    println!("{}", horizontal * depth); // 1840311528
}
