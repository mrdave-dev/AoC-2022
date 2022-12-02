use std::{fs};
use std::collections::HashMap;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Win,
    Loss,
    Draw,
}

struct RockPaperScissorsGame {}

impl RockPaperScissorsGame {
    fn get_winning_choice(opponent_choice: &RPSChoice) -> RPSChoice {
        match opponent_choice {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    fn get_losing_choice(opponent_choice: &RPSChoice) -> RPSChoice{
        match opponent_choice {
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
            RPSChoice::Scissors => RPSChoice::Paper,
        }
    }

    pub fn get_response(opponent_choice: &RPSChoice, desired_result: &str) -> RPSChoice {
        match desired_result {
            "X" => Self::get_losing_choice(opponent_choice),
            "Y" => *opponent_choice,
            "Z" => Self::get_winning_choice(opponent_choice),
            &_ => panic!("Expected a valid desired result (X, Y, or Z)"),
        }
    }

    /// Returns true if the user_choice wins, otherwise false
    fn determine_win(user_choice: &RPSChoice, opponent_choice: &RPSChoice) -> Result {
        if user_choice == opponent_choice {
            return Result::Draw;
        }

        return match user_choice {
            RPSChoice::Rock => if matches!(opponent_choice, RPSChoice::Scissors) { Result::Win } else { Result::Loss },
            RPSChoice::Paper => if matches!(opponent_choice, RPSChoice::Rock) { Result::Win } else { Result::Loss },
            RPSChoice::Scissors => if matches!(opponent_choice, RPSChoice::Paper) { Result::Win } else { Result::Loss },
        }
    }

    pub fn transform_input(choice: &str) -> RPSChoice {
        match choice {
            "A" | "X" => RPSChoice::Rock,
            "B" | "Y" => RPSChoice::Paper,
            "C" | "Z" => RPSChoice::Scissors,
            &_ => panic!("Expected a valid Rock, Paper, Scissors choice"),
        }
    }

    pub fn play(user_choice: &str, opponent_choice: &str) -> Result {
        let user_rps_choice = Self::transform_input(user_choice);
        let opponent_rps_choice = Self::transform_input(opponent_choice);
        return Self::determine_win(&user_rps_choice, &opponent_rps_choice);
    }

    pub fn play_rps(user_choice: &RPSChoice, opponent_choice: &RPSChoice) -> Result {
        return Self::determine_win(user_choice, opponent_choice);
    }
}

fn main() {
    let file_content = get_file_content();
    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut win_count: u32 = 0;
    let mut loss_count: u32 = 0;
    let mut draw_count: u32 = 0;
    let mut user_score: u32 = 0;

    let score_map = HashMap::from([
        (RPSChoice::Rock, 1),
        (RPSChoice::Paper, 2),
        (RPSChoice::Scissors, 3),
    ]);

    
    for line in lines {
        let choices: Vec<&str> = line.split(" ").collect();
        assert_eq!(choices.len(), 2);
        
        let opponent_rps_choice = RockPaperScissorsGame::transform_input(choices[0]);
        
        // For the first part: 
        // let user_rps_choice = RockPaperScissorsGame::transform_input(choices[1]);
        let user_rps_choice = RockPaperScissorsGame::get_response(&opponent_rps_choice, choices[1]);

        let user_choice_score = score_map.get(&user_rps_choice).unwrap();
        user_score += user_choice_score;

        match RockPaperScissorsGame::play_rps(&user_rps_choice, &opponent_rps_choice) {
            Result::Win => {
                win_count += 1;
                user_score += 6;
            },
            Result::Loss => {
                loss_count += 1;
            },
            Result::Draw => {
                draw_count += 1;
                user_score += 3;
            },
        }
    }

    println!("Wins:   {}", win_count);
    println!("Losses: {}", loss_count);
    println!("Draws:  {}", draw_count);
    println!("Score:  {}", user_score);
}
