use std::fs;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const OUTCOME_LOSE: char = 'X';
const OUTCOME_DRAW: char = 'Y';
const OUTCOME_WIN: char = 'Z';

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSS: i32 = 0;

fn get_score_by_outcome(outcome: char) -> i32 {
    if outcome == OUTCOME_WIN {
        return SCORE_WIN;
    }
    if outcome == OUTCOME_DRAW {
        return SCORE_DRAW;
    }
    if outcome == OUTCOME_LOSE {
        return SCORE_LOSS;
    }
    return 0;
}

fn get_score_shape_by_opponent_selection_and_outcome(
    outcome: char,
    opponent_selection: char,
) -> i32 {
    if outcome == OUTCOME_WIN && opponent_selection == OPPONENT_ROCK {
        return SCORE_PAPER;
    }
    if outcome == OUTCOME_WIN && opponent_selection == OPPONENT_PAPER {
        return SCORE_SCISSORS;
    }
    if outcome == OUTCOME_WIN && opponent_selection == OPPONENT_SCISSORS {
        return SCORE_ROCK;
    }

    if outcome == OUTCOME_DRAW {
        if opponent_selection == OPPONENT_ROCK {
            return SCORE_ROCK;
        }
        if opponent_selection == OPPONENT_PAPER {
            return SCORE_PAPER;
        }
        if opponent_selection == OPPONENT_SCISSORS {
            return SCORE_SCISSORS;
        }
    }

    if outcome == OUTCOME_LOSE {
        if opponent_selection == OPPONENT_ROCK {
            return SCORE_SCISSORS;
        }
        if opponent_selection == OPPONENT_PAPER {
            return SCORE_ROCK;
        }
        if opponent_selection == OPPONENT_SCISSORS {
            return SCORE_PAPER;
        }
    }
    panic!("Should't happen");
}

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut score_total = 0;

    for line in lines {
        let round_input: Vec<char> = line.chars().collect();
        let choice_opponent = round_input[0];
        let guide_outcome = round_input[2];

        let mut score_round_outcome = 0;
        let mut score_current_round = 0;

        score_round_outcome = get_score_by_outcome(guide_outcome);
        let score_shape =
            get_score_shape_by_opponent_selection_and_outcome(guide_outcome, choice_opponent);

        // update scores
        score_current_round += score_shape + score_round_outcome;
        score_total += score_current_round;
        println!(
            "{} {} {}",
            round_input[0], round_input[2], score_current_round
        );
    }

    println!("Total score: {}", score_total);
}
