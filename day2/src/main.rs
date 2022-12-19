const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn main() {
    let score = include_str!("input.txt")
        .lines()
        .map(|v| {
            let turn: Vec<&str> = v.split(' ').collect();
            score_round_part2(turn[0], turn[1])
        })
        .sum::<i32>();
    println!("score: {score:?}");
}

fn score_round_part1(them: &str, me: &str) -> i32 {
    match me {
        "X" => {
            ROCK + match them {
                // ROCK v ROCK.
                "A" => DRAW,
                // ROCK v PAPER.
                "B" => LOSE,
                // ROCK v SCISSORS.
                "C" => WIN,
                _ => panic!("invalid input: {}", them),
            }
        }
        "Y" => {
            PAPER
                + match them {
                    // PAPER v ROCK.
                    "A" => WIN,
                    // PAPER v PAPER.
                    "B" => DRAW,
                    // PAPER v SCISSORS.
                    "C" => LOSE,
                    _ => panic!("invalid input: {}", them),
                }
        }
        "Z" => {
            SCISSORS
                + match them {
                    // SCISSORS v ROCK.
                    "A" => LOSE,
                    // SCISSORS v PAPER.
                    "B" => WIN,
                    // SCISSORS v SCISSORS.
                    "C" => DRAW,
                    _ => panic!("invalid input: {}", them),
                }
        }
        _ => panic!("invalid input: {}", me),
    }
}

fn score_round_part2(them: &str, me: &str) -> i32 {
    match me {
        // Need to lose
        "X" => {
            LOSE + match them {
                // Rock beats scissors.
                "A" => SCISSORS,
                // Paper beats rock.
                "B" => ROCK,
                // Scissors beats paper.
                "C" => PAPER,
                _ => panic!("invalid input: {}", them),
            }
        }
        // Need to tie.
        "Y" => {
            DRAW + match them {
                // Rock.
                "A" => ROCK,
                // Paper.
                "B" => PAPER,
                // Scissors.
                "C" => SCISSORS,
                _ => panic!("invalid input: {}", them),
            }
        }
        // Need to win.
        "Z" => {
            WIN + match them {
                // Rock loses to paper.
                "A" => PAPER,
                // Paper loses to scissors.
                "B" => SCISSORS,
                // Scissors loses to rock.
                "C" => ROCK,
                _ => panic!("invalid input: {}", them),
            }
        }
        _ => panic!("invalid input: {}", me),
    }
}
