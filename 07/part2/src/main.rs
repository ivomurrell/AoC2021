use std::fs::read_to_string;

fn main() {
    let crabs: Vec<u32> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .next()
        .expect("input file was empty")
        .split(',')
        .map(|timer| timer.parse().expect("failed to parse crab position"))
        .collect();

    let (alignment, cost): (_, u32) = (*crabs.iter().min().expect("input file was empty")
        ..=*crabs.iter().max().expect("input file was empty"))
        .map(|alignment| {
            crabs
                .iter()
                .map(|&position| {
                    let diff = if position > alignment {
                        position - alignment
                    } else {
                        alignment - position
                    };
                    (diff * (diff + 1)) / 2
                })
                .sum()
        })
        .enumerate()
        .min_by_key(|&(_, cost)| cost)
        .expect("input file was empty");

    println!(
        "The cheapest position for the crabs to align to is {} with a cost of {}!",
        alignment, cost
    );
}
