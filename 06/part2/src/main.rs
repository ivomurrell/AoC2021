use std::fs::read_to_string;

fn main() {
    let mut timers = [0u64; 9];
    read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .next()
        .expect("input file was empty")
        .split(',')
        .map(|timer| timer.parse().expect("failed to parse lanternfish timer"))
        .for_each(|fish_timer: usize| timers[fish_timer] += 1);

    for _ in 0..256 {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }

    let fish_count: u64 = timers.iter().sum();
    println!(
        "After 256 days there is now a total of {} lanternfish!",
        fish_count
    );
}
