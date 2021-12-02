use std::fs::read_to_string;

fn main() {
    let (hor_pos, depth, _) = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .fold((0, 0, 0), |(hor_pos, depth, aim), command| {
            let (direction, magnitude) = command
                .split_once(' ')
                .expect("failed to split command into parts");
            let magnitude: i32 = magnitude.parse().expect("failed to parse magnitude");
            match direction {
                "forward" => (hor_pos + magnitude, depth + magnitude * aim, aim),
                "down" => (hor_pos, depth, aim + magnitude),
                "up" => (hor_pos, depth, aim - magnitude),
                _ => panic!("unexpected direction '{}'", direction),
            }
        });
    println!(
        "The final horizontal position is {} and the final depth is {}, so the result is {}!",
        hor_pos,
        depth,
        hor_pos * depth
    );
}
