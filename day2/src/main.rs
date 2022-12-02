use std::fs;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const PLAYER_ROCK: char = 'X';
const PLAYER_PAPER: char = 'Y';
const PLAYER_SCISSORS: char = 'Z';

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSS: i32 = 0;

fn check_draw(shape_opponent: char, shape_player: char) -> bool {
    if shape_opponent == OPPONENT_PAPER && shape_player == PLAYER_PAPER {
        return true;
    }
    if shape_opponent == OPPONENT_ROCK && shape_player == PLAYER_ROCK {
        return true;
    }
    if shape_opponent == OPPONENT_SCISSORS && shape_player == PLAYER_SCISSORS {
        return true;
    }
    return false;
}

fn check_win(shape_opponent: char, shape_player: char) -> bool {
    if shape_player == PLAYER_ROCK && shape_opponent == OPPONENT_SCISSORS {
        return true;
    }
    if shape_player == PLAYER_PAPER && shape_opponent == OPPONENT_ROCK {
        return true;
    }
    if shape_player == PLAYER_SCISSORS && shape_opponent == OPPONENT_PAPER {
        return true;
    }
    return false;
}

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut score_total = 0;

    for line in lines {
        let round_input: Vec<char> = line.chars().collect();
        let choice_opponent = round_input[0];
        let choice_guide = round_input[2];

        let mut score_shape = 0;
        let mut score_round_outcome = 0;
        let mut score_current_round = 0;

        // calculate shapes score
        if choice_guide == PLAYER_ROCK {
            score_shape += SCORE_ROCK;
        } else if choice_guide == PLAYER_PAPER {
            score_shape += SCORE_PAPER;
        } else if choice_guide == PLAYER_SCISSORS {
            score_shape += SCORE_SCISSORS;
        }

        if check_draw(choice_opponent, choice_guide) {
            score_round_outcome = SCORE_DRAW;
        } else if check_win(choice_opponent, choice_guide) {
            score_round_outcome = SCORE_WIN;
        } else {
            score_round_outcome = SCORE_LOSS;
        }

        // update scores
        score_current_round += score_shape + score_round_outcome;
        score_total += score_current_round;
        println!("{} {}", round_input[0], round_input[2]);
    }

    println!("Total score: {}", score_total);
}
