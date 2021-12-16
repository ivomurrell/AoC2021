use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let length = input.lines().next().expect("input was empty").len();
    let mut unvisited: HashSet<_> = (0..length)
        .flat_map(|x| (0..length).map(move |y| (x, y)))
        .collect();
    let mut risk_paths: HashMap<_, _> = unvisited.iter().map(|&coord| (coord, u32::MAX)).collect();
    let risk_levels: HashMap<_, _> = input
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

    let mut visiting = (0, 0);
    risk_paths.insert(visiting, 0);
    while visiting != (length - 1, length - 1) {
        unvisited.remove(&visiting);
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
            .filter(|neighbour| unvisited.contains(neighbour))
        {
            let path_risk = visited_path + risk_levels[&unvisited_neighbour];
            if path_risk < risk_paths[&unvisited_neighbour] {
                risk_paths.insert(unvisited_neighbour, path_risk);
            }
        }

        visiting = *unvisited
            .iter()
            .min_by_key(|unvisited_coord| risk_paths[unvisited_coord])
            .expect("no unvisited coordinates positions left");
    }

    let lowest_risk = risk_paths[&(length - 1, length - 1)];
    println!("The lowest total risk of any path is {}!", lowest_risk);
}
