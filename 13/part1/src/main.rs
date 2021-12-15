use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let (dots, instructions) = input.split_once("\n\n").expect("failed to parse file");

    let dots: HashSet<(u32, u32)> = dots
        .lines()
        .map(|dot| {
            let (x, y) = dot.split_once(',').expect("failed to parse dot");
            (
                x.parse().expect("failed to parse x co-ordinate"),
                y.parse().expect("failed to parse y co-ordinate"),
            )
        })
        .collect();

    let instruction = instructions
        .lines()
        .next()
        .expect("failed to read any instructions");
    let axis = &instruction[11..12];
    let line: u32 = instruction[13..].parse().expect("failed to parse line");

    let folded_dots: HashSet<_> = match axis {
        "x" => dots
            .into_iter()
            .map(
                |dot @ (x, y)| {
                    if x > line {
                        (line * 2 - x, y)
                    } else {
                        dot
                    }
                },
            )
            .collect(),
        "y" => dots
            .into_iter()
            .map(
                |dot @ (x, y)| {
                    if y > line {
                        (x, line * 2 - y)
                    } else {
                        dot
                    }
                },
            )
            .collect(),
        _ => panic!("unrecognised axis {}!", axis),
    };

    println!(
        "There are {} visible dots after the first fold!",
        folded_dots.len()
    );
}
