use std::{collections::BTreeSet, fs::read_to_string};

fn main() {
    let mut covered = BTreeSet::new();
    let mut overlapping = BTreeSet::new();
    read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .filter_map(|line| {
            let (a, b) = line
                .split_once(" -> ")
                .expect("failed to find two co-ordinates");

            fn parse_coordinate(coord: &str) -> (u32, u32) {
                let (x, y) = coord.split_once(',').expect("failed to read co-ordinate");
                let x = x.parse().expect("failed to parse x co-ordinate");
                let y = y.parse().expect("failed to parse y co-ordinate");
                (x, y)
            }
            let a = parse_coordinate(a);
            let b = parse_coordinate(b);

            if a.0 == b.0 {
                Some(
                    if a.1 > b.1 { b.1..=a.1 } else { a.1..=b.1 }
                        .map(|y| (a.0, y))
                        .collect(),
                )
            } else if a.1 == b.1 {
                Some(
                    if a.0 > b.0 { b.0..=a.0 } else { a.0..=b.0 }
                        .map(|x| (x, a.1))
                        .collect(),
                )
            } else {
                None
            }
        })
        .for_each(|line: Vec<_>| {
            for point in line.into_iter() {
                if !covered.insert(point) {
                    overlapping.insert(point);
                }
            }
        });
    println!("Vents overlap at {} points!", overlapping.len());
}
