use std::fs::read_to_string;

fn main() {
    let depths: Vec<u32> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .map(|depth| depth.parse().expect("failed to parse depth"))
        .collect();
    let increases = depths
        .windows(4)
        .filter(|windowed_depths| {
            let a: u32 = windowed_depths[0..3].iter().sum();
            let b: u32 = windowed_depths[1..4].iter().sum();
            b > a
        })
        .count();
    println!(
        "The number of times the sliding window increases is {}!",
        increases
    )
}
