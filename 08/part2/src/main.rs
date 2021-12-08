#![allow(clippy::many_single_char_names)]

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let seg_range = 'a'..='g';
    let digit_segments: Vec<(HashSet<_>, _)> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, segments)| (segments.chars().collect(), i))
    .collect();

    let output_sum: usize = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .map(|entry| {
            let mut identified = HashMap::new();
            let (patterns, output) = entry.split_once('|').expect("failed to parse entry");
            let patterns: Vec<_> = patterns.split_ascii_whitespace().collect();

            let seven = patterns
                .iter()
                .find(|pattern| pattern.len() == 3)
                .expect("failed to find 7 digit");
            let one = patterns
                .iter()
                .find(|pattern| pattern.len() == 2)
                .expect("failed to find 1 digit");
            let a = seven
                .chars()
                .find(|&seg| !one.contains(seg))
                .expect("failed to find 'd' candidate");
            identified.insert(a, 'a');

            let e_test_string: String = patterns
                .iter()
                .filter(|pattern| pattern.len() == 5 || pattern.len() == 6)
                .copied()
                .collect();
            let e = seg_range
                .clone()
                .find(|&seg| e_test_string.matches(seg).count() == 3)
                .expect("failed to find 'e' candidate");
            identified.insert(e, 'e');

            let zero_six: Vec<_> = patterns
                .iter()
                .filter(|pattern| pattern.len() == 6 && pattern.contains(e))
                .collect();
            let zero_index = zero_six
                .iter()
                .position(|pattern| one.chars().all(|seg| pattern.contains(seg)))
                .expect("failed to find 0 index");
            let zero = zero_six[zero_index];
            let six = zero_six[1 - zero_index];
            let c = seg_range
                .clone()
                .find(|&seg| !six.contains(seg))
                .expect("failed to find 'c' candidate");
            identified.insert(c, 'c');
            let d = seg_range
                .clone()
                .find(|&seg| !zero.contains(seg))
                .expect("failed to find 'd' candidate");
            identified.insert(d, 'd');

            let f = one
                .chars()
                .find(|&seg| seg != c)
                .expect("failed to find 'f' candidate");
            identified.insert(f, 'f');

            let four = patterns
                .iter()
                .find(|pattern| pattern.len() == 4)
                .expect("failed to find 4 digit");
            let b = four
                .chars()
                .find(|&seg| seg != c && seg != d && seg != f)
                .expect("failed to find 'b' candidate");
            identified.insert(b, 'b');

            let g = seg_range
                .clone()
                .find(|seg| !identified.contains_key(seg))
                .expect("failed to find 'g' candidate");
            identified.insert(g, 'g');

            let output: usize = output
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, digit)| {
                    let digit: HashSet<_> = digit.chars().map(|seg| identified[&seg]).collect();
                    let digit = digit_segments
                        .iter()
                        .find(|&(candidate, _)| digit == *candidate)
                        .expect("failed to find matching digit")
                        .1;
                    digit * 10usize.pow(3 - i as u32)
                })
                .sum();
            output
        })
        .sum();

    println!("The output values sum to {}!", output_sum);
}
