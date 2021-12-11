use std::{collections::HashSet, fs::read_to_string, ops::RangeInclusive};

fn get_neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    fn clamped_range(i: usize) -> RangeInclusive<usize> {
        (if i == 0 { 0 } else { i - 1 })..=(if i == 9 { 9 } else { i + 1 })
    }

    clamped_range(x)
        .flat_map(move |x| clamped_range(y).map(move |y| (x, y)))
        .filter(move |&point| point != (x, y))
}

fn gain_energy(
    energy_levels: &mut [[u32; 10]; 10],
    flashed: &mut HashSet<(usize, usize)>,
    point @ (x, y): (usize, usize),
) {
    let octopus = &mut energy_levels[y][x];
    *octopus += 1;

    if *octopus > 9 {
        let first_flash = flashed.insert(point);
        if first_flash {
            for neighbour in get_neighbours(x, y) {
                gain_energy(energy_levels, flashed, neighbour);
            }
        }
    }
}

fn main() {
    let mut energy_levels = [[0; 10]; 10];
    let input = read_to_string("../input").expect("failed to open input file");
    for (y, line) in input.lines().enumerate() {
        for (x, octopus) in line.chars().enumerate() {
            let energy = octopus
                .to_digit(10)
                .expect("failed to parse octopus energy level");
            energy_levels[y][x] = energy;
        }
    }

    let mut step = 1;
    loop {
        let mut flashed = HashSet::new();
        for x in 0..10 {
            for y in 0..10 {
                gain_energy(&mut energy_levels, &mut flashed, (x, y))
            }
        }
        if flashed.len() == 100 {
            break;
        }
        for (x, y) in flashed {
            energy_levels[y][x] = 0
        }
        step += 1;
    }

    println!("All octopi will flash together on step {}!", step);
}
