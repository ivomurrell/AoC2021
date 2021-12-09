use std::fs::read_to_string;

fn main() {
    let heatmap: Vec<Vec<_>> = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).expect("failed to parse height"))
                .collect()
        })
        .collect();
    let risk_total: u32 = heatmap
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &height)| (height, x, y))
        })
        .filter_map(|(height, x, y)| {
            let up = y.checked_sub(1).map(|y| heatmap[y][x]);
            let left = x.checked_sub(1).map(|x| heatmap[y][x]);
            let right = heatmap[y].get(x + 1).copied();
            let down = heatmap.get(y + 1).map(|line| line[x]);
            if [up, left, right, down].into_iter().all(|neighbour| {
                neighbour
                    .map(|neighbour| neighbour > height)
                    .unwrap_or(true)
            }) {
                Some(height + 1)
            } else {
                None
            }
        })
        .sum();
    println!("The sum of all risk levels is {}!", risk_total);
}
