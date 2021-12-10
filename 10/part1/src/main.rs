use std::fs::read_to_string;

fn main() {
    let error_score: u32 = read_to_string("../input")
        .expect("failed to open input file")
        .lines()
        .filter_map(|line| {
            let mut chunks = Vec::new();
            fn pop_chunk(chunks: &mut Vec<char>) -> char {
                chunks.pop().expect("found closing chunk without opening")
            }

            for chunk in line.chars() {
                match chunk {
                    '(' | '[' | '{' | '<' => chunks.push(chunk),
                    ')' => {
                        if pop_chunk(&mut chunks) != '(' {
                            return Some(3);
                        }
                    }
                    ']' => {
                        if pop_chunk(&mut chunks) != '[' {
                            return Some(57);
                        }
                    }
                    '}' => {
                        if pop_chunk(&mut chunks) != '{' {
                            return Some(1197);
                        }
                    }
                    '>' => {
                        if pop_chunk(&mut chunks) != '<' {
                            return Some(25137);
                        }
                    }
                    _ => panic!("Unrecognised chunk character '{}'", chunk),
                }
            }
            None
        })
        .sum();
    println!("The total syntax error score is {}!", error_score);
}
