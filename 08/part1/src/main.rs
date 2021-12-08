use std::fs::read_to_string;

fn main() {
    let unique_segment_count = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .flat_map(|entry| {
            entry
                .split_once('|')
                .expect("failed to parse entry")
                .1
                .split_ascii_whitespace()
        })
        .filter(|output| {
            let segments = output.len();
            segments == 2 || segments == 3 || segments == 4 || segments == 7
        })
        .count();
    println!(
        "The digits 1, 4, 7, and 8 appear {} times!",
        unique_segment_count
    );
}
