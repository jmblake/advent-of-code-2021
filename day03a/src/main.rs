use std::fs;

const NUM_BITS: usize = 12;
const NUM_ROWS: u32 = 1000;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|x| String::from(x))
        .collect();
    let mut counts: [u32; NUM_BITS] = [0; NUM_BITS];
    for i in 0..NUM_BITS {
        counts[i] = input.iter().filter(|&s| s.chars().nth(i).unwrap() == '1').count() as u32;
    }
    let gamma_str = counts.iter().map(|&x| if x > NUM_ROWS / 2 {"1"} else {"0"}).collect::<Vec<_>>().join("");
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = !gamma & ((1 << NUM_BITS) - 1); // https://stackoverflow.com/a/36124832
    println!("{}", gamma * epsilon);
}
