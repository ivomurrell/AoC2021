use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let tile_length = input.lines().next().expect("input was empty").len();
    let length = tile_length * 5;
    let mut risk_paths: HashMap<_, _> = (0..length)
        .flat_map(|x| (0..length).map(move |y| ((x, y), u32::MAX)))
        .collect();
    let tile_risk_levels: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, risk)| {
                (
                    (x, y),
                    risk.to_digit(10).expect("failed to parse risk level"),
                )
            })
        })
        .collect();
    let tile_risk_levels = &tile_risk_levels;
    let risk_levels: HashMap<_, _> = (0..5)
        .flat_map(|a| {
            (0..5).flat_map(move |b| {
                tile_risk_levels.iter().map(move |((x, y), risk)| {
                    let new_risk = risk + a as u32 + b as u32;
                    let new_risk = if new_risk > 9 { new_risk - 9 } else { new_risk };
                    ((x + tile_length * a, y + tile_length * b), new_risk)
                })
            })
        })
        .collect();

    let mut visited = HashSet::new();
    let mut unvisited_neighbours = HashSet::new();
    let mut visiting = (0, 0);
    risk_paths.insert(visiting, 0);
    while visiting != (length - 1, length - 1) {
        unvisited_neighbours.remove(&visiting);
        visited.insert(visiting);
        let visited_path = risk_paths[&visiting];

        let (x, y) = visiting;
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < length - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < length - 1 {
            neighbours.push((x, y + 1));
        }

        for unvisited_neighbour in neighbours
            .into_iter()
            .filter(|neighbour| !visited.contains(neighbour))
        {
            unvisited_neighbours.insert(unvisited_neighbour);
            let path_risk = visited_path + risk_levels[&unvisited_neighbour];
            if path_risk < risk_paths[&unvisited_neighbour] {
                risk_paths.insert(unvisited_neighbour, path_risk);
            }
        }

        visiting = *unvisited_neighbours
            .iter()
            .min_by_key(|unvisited_coord| risk_paths[unvisited_coord])
            .expect("no unvisited coordinates positions left");
    }

    let lowest_risk = risk_paths[&(length - 1, length - 1)];
    println!("The lowest total risk of any path is {}!", lowest_risk);
}
