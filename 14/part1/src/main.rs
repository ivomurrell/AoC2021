use std::{collections::HashMap, fs::read_to_string, iter, str};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let (template, insertions) = input.split_once("\n\n").expect("failed to parse file");

    let template = template.to_owned().into_bytes();
    let rules: HashMap<_, _> = insertions
        .lines()
        .map(|rule| rule.split_once(" -> ").expect("failed to parse rule"))
        .collect();

    let polymer = (0..10).fold(template, |polymer, _| {
        polymer
            .windows(2)
            .into_iter()
            .flat_map(|elements| {
                let new = rules
                    .get(str::from_utf8(elements).expect("failed to create pair"))
                    .expect("failed to find pair in rules");
                [elements[0], new.as_bytes()[0]]
            })
            .chain(iter::once(*polymer.last().expect("polymer was empty")))
            .collect()
    });

    let mut element_count: HashMap<_, u32> = HashMap::new();
    for element in polymer {
        *element_count.entry(element).or_default() += 1;
    }

    let max_element = element_count.values().max().expect("polymer was empty");
    let min_element = element_count.values().min().expect("polymer was empty");
    println!(
        "The difference between the most common and least common element is {}!",
        max_element - min_element
    );
}
