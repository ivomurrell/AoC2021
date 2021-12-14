use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let caves = input.lines().fold(
        HashMap::new(),
        |mut caves: HashMap<_, Vec<_>>, connection| {
            let (from, to) = connection
                .split_once("-")
                .expect("failed to parse connection");
            caves.entry(from).or_default().push(to);
            caves.entry(to).or_default().push(from);
            caves
        },
    );

    fn explore_cave<'a>(
        caves: &HashMap<&'a str, Vec<&'a str>>,
        mut small_visited: HashSet<&'a str>,
        current: &'a str,
    ) -> Vec<Vec<&'a str>> {
        if current == "end" {
            return vec![vec![current]];
        }

        if current
            .chars()
            .next()
            .expect("current cave string was empty")
            .is_lowercase()
        {
            small_visited.insert(current);
        }

        if let Some(connections) = caves.get(current) {
            connections
                .iter()
                .filter(|&cave| !small_visited.contains(cave))
                .flat_map(|cave| {
                    let mut paths = explore_cave(caves, small_visited.clone(), cave);
                    for path in paths.iter_mut() {
                        path.push(current);
                    }
                    paths
                })
                .filter(|path| *path.first().expect("path was empty") == "end")
                .collect()
        } else {
            vec![]
        }
    }

    let paths = explore_cave(&caves, HashSet::new(), "start");
    println!("There are {} paths through the cave system!", paths.len());
}
