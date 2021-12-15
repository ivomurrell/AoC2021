use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let (template, insertions) = input.split_once("\n\n").expect("failed to parse file");

    let mut template_chars = template.bytes();
    let first_element = template_chars.next().expect("template was empty");
    let last_element = template_chars.last().expect("template was too short");

    let mut template_pair_count = HashMap::new();
    for pair in template.as_bytes().windows(2) {
        let pair = pair.to_vec();
        *template_pair_count.entry(pair).or_default() += 1;
    }

    let rules: HashMap<_, _> = insertions
        .lines()
        .map(|rule| {
            let (key, val) = rule.split_once(" -> ").expect("failed to parse rule");
            (key.as_bytes(), val.as_bytes()[0])
        })
        .collect();

    let pair_count = (0..40).fold(template_pair_count, |pair_count, _| {
        let mut next_pair_count: HashMap<_, u64> = HashMap::new();
        for (pair, count) in pair_count {
            let new_element = rules[pair.as_slice()];
            let mut interspersed = pair.clone();
            interspersed.insert(1, new_element);
            for new_pair in [&interspersed[0..2], &interspersed[1..3]] {
                *next_pair_count.entry(new_pair.to_owned()).or_default() += count;
            }
        }
        next_pair_count
    });

    let mut element_count: HashMap<_, u64> = HashMap::new();
    for (pair, count) in pair_count {
        for element in pair {
            *element_count.entry(element).or_default() += count;
        }
    }
    for count in element_count.values_mut() {
        *count /= 2;
    }
    *element_count.entry(first_element).or_default() += 1;
    *element_count.entry(last_element).or_default() += 1;

    let max_element = element_count.values().max().expect("polymer was empty");
    let min_element = element_count.values().min().expect("polymer was empty");
    println!(
        "The difference between the most common and least common element is {}!",
        max_element - min_element
    );
}
