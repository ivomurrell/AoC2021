use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn is_small_cave(cave: &str) -> bool {
    cave.chars()
        .next()
        .expect("current cave string was empty")
        .is_lowercase()
}

fn explore_cave<'a>(
    caves: &HashMap<&'a str, Vec<&'a str>>,
    mut small_visited: HashSet<&'a str>,
    special_cave: &'a str,
    current: &'a str,
) -> Vec<Vec<&'a str>> {
    if current == "end" {
        return vec![vec![current]];
    }

    if current == special_cave {
        let already_visited_once = !small_visited.insert("special-visited");
        if already_visited_once {
            small_visited.insert(current);
        }
    } else if is_small_cave(current) {
        small_visited.insert(current);
    }

    if let Some(connections) = caves.get(current) {
        connections
            .iter()
            .filter(|&cave| !small_visited.contains(cave))
            .flat_map(|cave| {
                let mut paths = explore_cave(caves, small_visited.clone(), special_cave, cave);
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

    let paths: HashSet<_> = caves
        .keys()
        .filter(|&&cave| cave != "start" && cave != "end" && is_small_cave(cave))
        .flat_map(|special_cave| explore_cave(&caves, HashSet::new(), special_cave, "start"))
        .collect();
    println!("There are {} paths through the cave system!", paths.len());
}
