use std::fs::File;
use std::io::{self, Read};

pub fn run_rps_tournament_part_one() -> u32 {
    let mut file = File::open("utils/rps_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut current_score = 0;

    for word in words {
        let parts = word.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        let points = calculate_points(parts);

        current_score = current_score + points;
    }
    return current_score;
}

pub fn run_rps_tournament_part_two() -> u32 {
    let mut file = File::open("utils/rps_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut current_score = 0;

    for word in words {
        let parts = word.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        let points = calculate_points_part_two(parts);

        current_score = current_score + points;
    }

    return current_score;
}

fn calculate_points(game_parts: Vec<&str>) -> u32 {
    let mut points = 0;
    let player_one_letter = game_parts[0];
    let player_two_letter = game_parts[1];

    let player_one_sign = match player_one_letter {
        "A" => "Rock",
        "B" => "Paper",
        "C" => "Scissors",
        _ => "Unknown",
    };

    let player_two_sign = match player_two_letter {
        "X" => "Rock",
        "Y" => "Paper",
        "Z" => "Scissors",
        _ => "Unknown",
    };

    let player_one_value = match player_one_letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };

    let player_two_value = match player_two_letter {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    if player_one_sign == "Rock" {
        match player_two_sign {
            "Rock" => {
                // a tie
                points = player_two_value + 3;
            }
            "Paper" => {
                // a win
                points = player_two_value + 6;
            }
            "Scissors" => {
                // a loss
                points = player_two_value;
            }
            _ => {}
        }
    } else if player_one_sign == "Paper" {
        match player_two_sign {
            "Rock" => {
                // a loss
                points = player_two_value;
            }
            "Paper" => {
                // a tie
                points = player_two_value + 3;
            }
            "Scissors" => {
                // a win
                points = player_two_value + 6;
            }
            _ => {}
        }
    } else if player_one_sign == "Scissors" {
        match player_two_sign {
            "Rock" => {
                // a win
                points = player_two_value + 6;
            }
            "Paper" => {
                // a loss
                points = player_two_value;
            }
            "Scissors" => {
                // a tie
                points = player_two_value + 3;
            }
            _ => {}
        }
    }

    return points;
}

fn calculate_points_part_two(game_parts: Vec<&str>) -> u32 {
    let mut points = 0;
    let player_one_letter = game_parts[0];
    let outcome_letter = game_parts[1];

    let player_one_sign = match player_one_letter {
        "A" => "Rock",
        "B" => "Paper",
        "C" => "Scissors",
        _ => "Unknown",
    };

    let outcome = match outcome_letter {
        "X" => "Lose",
        "Y" => "Tie",
        "Z" => "Win",
        _ => "Unknown",
    };

    let player_one_value = match player_one_letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };

    let player_two_value = determine_player_two_value(player_one_sign, outcome);

    if player_one_sign == "Rock" {
        match player_two_value {
            1 => {
                // a tie
                points = player_two_value + 3;
            }
            2 => {
                // a win
                points = player_two_value + 6;
            }
            3 => {
                // a loss
                points = player_two_value;
            }
            _ => {}
        }
    } else if player_one_sign == "Paper" {
        match player_two_value {
            1 => {
                // a loss
                points = player_two_value;
            }
            2 => {
                // a tie
                points = player_two_value + 3;
            }
            3 => {
                // a win
                points = player_two_value + 6;
            }
            _ => {}
        }
    } else if player_one_sign == "Scissors" {
        match player_two_value {
            1 => {
                // a win
                points = player_two_value + 6;
            }
            2 => {
                // a loss
                points = player_two_value;
            }
            3 => {
                // a tie
                points = player_two_value + 3;
            }
            _ => {}
        }
    }

    return points;
}

fn determine_player_two_value(player_one_move: &str, outcome: &str) -> u32 {
    let player_two_value = match player_one_move {
        "Rock" => match outcome {
            "Lose" => 3,
            "Tie" => 1,
            "Win" => 2,
            _ => 0,
        },
        "Paper" => match outcome {
            "Lose" => 1,
            "Tie" => 2,
            "Win" => 3,
            _ => 0,
        },
        "Scissors" => match outcome {
            "Lose" => 2,
            "Tie" => 3,
            "Win" => 1,
            _ => 0,
        },
        _ => 0,
    };

    return player_two_value;
}
