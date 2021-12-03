use std::fs::read_to_string;

fn iterate_criteria(candidates: Vec<u32>, is_least: bool, bit: usize) -> u32 {
    let (zeroes, ones): (Vec<_>, _) = candidates
        .into_iter()
        .partition(|&candidate| (candidate & (1 << bit)) == 0);
    let met = if (zeroes.len() > ones.len()) ^ is_least {
        zeroes
    } else {
        ones
    };
    if met.len() > 1 {
        iterate_criteria(met, is_least, bit - 1)
    } else {
        met[0]
    }
}

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let size = input.find('\n').expect("failed to find first newline");
    let report: Vec<_> = input
        .lines()
        .map(|report_num| {
            u32::from_str_radix(report_num, 2).expect("failed to parse number as binary")
        })
        .collect();
    let oxygen_generator = iterate_criteria(report.clone(), false, size - 1);
    let co2_scrubber = iterate_criteria(report, true, size - 1);
    let life_support = oxygen_generator * co2_scrubber;
    println!(
        "The oxygen generator rating is {} and the CO2 scrubber rating is {} so the life support rating is {}!",
        oxygen_generator,
        co2_scrubber,
        life_support
    );
}
