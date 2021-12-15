use std::{collections::BTreeSet, fs::read_to_string};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let (dots, instructions) = input.split_once("\n\n").expect("failed to parse file");

    let initial_dots: BTreeSet<(usize, usize)> = dots
        .lines()
        .map(|dot| {
            let (x, y) = dot.split_once(',').expect("failed to parse dot");
            (
                y.parse().expect("failed to parse y co-ordinate"),
                x.parse().expect("failed to parse x co-ordinate"),
            )
        })
        .collect();

    let folded_dots = instructions
        .lines()
        .fold(initial_dots, |dots, instruction| {
            let axis = &instruction[11..12];
            let line = instruction[13..].parse().expect("failed to parse line");
            match axis {
                "x" => dots
                    .into_iter()
                    .map(|dot @ (y, x)| if x > line { (y, line * 2 - x) } else { dot })
                    .collect(),
                "y" => dots
                    .into_iter()
                    .map(|dot @ (y, x)| if y > line { (line * 2 - y, x) } else { dot })
                    .collect(),
                _ => panic!("unrecognised axis {}!", axis),
            }
        });

    let mut cursor = (0, 0);
    for (y, x) in folded_dots {
        if y > cursor.1 {
            println!();
            cursor = (0, y);
        }
        if x > cursor.0 {
            print!("{}", ".".repeat(x - cursor.0 - 1));
            cursor.0 = x;
        }
        print!("#");
    }
}
