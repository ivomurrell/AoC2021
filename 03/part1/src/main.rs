use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let size = input.find('\n').expect("failed to find first newline");
    let mut bit_count = vec![0; size];
    for report_num in input.lines() {
        for (pos, bit) in report_num.chars().enumerate() {
            match bit {
                '1' => bit_count[pos] += 1,
                '0' => bit_count[pos] -= 1,
                _ => panic!("unrecognised bit '{}'", bit),
            }
        }
    }
    let gamma = bit_count.iter().enumerate().fold(0, |gamma, (pos, bit)| {
        if *bit > 0 {
            gamma | (1 << (size - pos - 1))
        } else {
            gamma
        }
    });
    let epsilon = !gamma & ((1 << size) - 1);
    println!(
        "The gamma rate is {} and the epsilon rate is {} so the result is {}!",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
