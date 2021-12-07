use std::fs::read_to_string;

fn main() {
    let mut crabs: Vec<u32> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .next()
        .expect("input file was empty")
        .split(',')
        .map(|timer| timer.parse().expect("failed to parse crab position"))
        .collect();
    crabs.sort_unstable();

    let crab_count = crabs.len();
    let mid = crab_count / 2;
    let median = if crab_count % 2 == 0 {
        f32::round((crabs[mid - 1] + crabs[mid]) as f32 / 2.0) as u32
    } else {
        crabs[mid]
    };

    let cost: u32 = crabs
        .iter()
        .map(|&position| {
            if position > median {
                position - median
            } else {
                median - position
            }
        })
        .sum();

    println!(
        "The cheapest position for the crabs to align to is {} with a cost of {}!",
        median, cost
    );
}
