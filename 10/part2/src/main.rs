use std::fs::read_to_string;

fn main() {
    let mut completion_scores: Vec<_> = read_to_string("../input")
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
                            return None;
                        }
                    }
                    ']' => {
                        if pop_chunk(&mut chunks) != '[' {
                            return None;
                        }
                    }
                    '}' => {
                        if pop_chunk(&mut chunks) != '{' {
                            return None;
                        }
                    }
                    '>' => {
                        if pop_chunk(&mut chunks) != '<' {
                            return None;
                        }
                    }
                    _ => panic!("Unrecognised chunk character '{}'", chunk),
                }
            }
            Some(chunks)
        })
        .map(|chunks| {
            chunks.into_iter().rfold(0u64, |score, chunk| {
                let score = score * 5;
                score
                    + match chunk {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unrecognised chunk character '{}'", chunk),
                    }
            })
        })
        .collect();

    completion_scores.sort_unstable();
    let middle = completion_scores[completion_scores.len() / 2];
    println!("The middle autocompletion score is {}!", middle);
}
