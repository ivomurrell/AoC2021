use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").expect("failed to open input file");
    let mut items = input.split("\n\n");
    let balls = items
        .next()
        .expect("input file was empty")
        .split(',')
        .map(|ball| ball.parse().expect("failed to parse ball number"));
    let mut boards: Vec<_> = items
        .map(|board_nums| {
            let mut board: [[Option<u32>; 5]; 5] = Default::default();
            for (line_i, line) in board_nums.lines().enumerate() {
                for (row_i, num) in line.split_ascii_whitespace().enumerate() {
                    let num = num.parse().expect("failed to parse board number");
                    board[line_i][row_i] = Some(num);
                }
            }
            board
        })
        .collect();

    for ball in balls {
        for board in boards.iter_mut() {
            for num in board.iter_mut().flat_map(|line| line.iter_mut()) {
                if num == &Some(ball) {
                    *num = None
                }
            }
        }

        let mut i = 0;
        while i < boards.len() {
            let board = boards[i];
            let bingo_row = board.iter().any(|line| *line == [None; 5]);
            let bingo_col = (0..5).any(|i| board.iter().all(|line| line[i] == None));

            if bingo_row || bingo_col {
                if boards.len() == 1 {
                    let board = boards[0];
                    let unmarked_sum: u32 = board
                        .iter()
                        .flat_map(|line| line.iter())
                        .filter_map(|num| *num)
                        .sum();
                    println!(
                        "The sum of all unmarked numbers on the last board is {} and the final ball is {} so the score is {}!",
                        unmarked_sum,
                        ball,
                        unmarked_sum * ball
                    );
                    return;
                }

                boards.remove(i);
            } else {
                i += 1
            }
        }
    }
}
