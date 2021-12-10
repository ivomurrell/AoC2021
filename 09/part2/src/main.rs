use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let heightmap: Vec<Vec<_>> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).expect("failed to parse height"))
                .collect()
        })
        .collect();

    let mut basin_sizes: Vec<_> = heightmap
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &height)| (height, x, y))
        })
        .filter_map(|(height, x, y)| {
            let up = y.checked_sub(1).map(|y| heightmap[y][x]);
            let left = x.checked_sub(1).map(|x| heightmap[y][x]);
            let right = heightmap[y].get(x + 1).copied();
            let down = heightmap.get(y + 1).map(|line| line[x]);
            if [up, left, right, down].into_iter().all(|neighbour| {
                neighbour
                    .map(|neighbour| neighbour > height)
                    .unwrap_or(true)
            }) {
                Some((x, y))
            } else {
                None
            }
        })
        .map(|low_point| {
            fn check_neighbours(
                heightmap: &[Vec<u32>],
                basin: &mut HashSet<(usize, usize)>,
                point @ (x, y): (usize, usize),
            ) {
                if heightmap
                    .get(y)
                    .and_then(|line| line.get(x))
                    .filter(|&&height| height != 9)
                    .is_some()
                {
                    let not_visited = basin.insert(point);
                    if not_visited {
                        if y > 0 {
                            check_neighbours(heightmap, basin, (x, y - 1));
                        }
                        if x > 0 {
                            check_neighbours(heightmap, basin, (x - 1, y));
                        }
                        check_neighbours(heightmap, basin, (x + 1, y));
                        check_neighbours(heightmap, basin, (x, y + 1));
                    }
                }
            }
            let mut basin = HashSet::new();
            check_neighbours(&heightmap, &mut basin, low_point);
            basin.len()
        })
        .collect();

    basin_sizes.sort_unstable();
    let result: usize = basin_sizes.into_iter().rev().take(3).product();
    println!("The product of the three largest basins is {}!", result);
}
