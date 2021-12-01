use std::fs::read_to_string;

fn main() {
    let depths: Vec<u32> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .map(|depth| depth.parse().expect("failed to parse depth"))
        .collect();
    let increases = depths
        .windows(2)
        .filter(|windowed_depths| windowed_depths[1] > windowed_depths[0])
        .count();
    println!(
        "The number of times the depth measurement increases is {}!",
        increases
    )
}
