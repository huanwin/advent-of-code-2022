use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Outcome {
    Win,
    Loss,
    Draw
}

enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

struct GameRound {
    opponent: Shape,
    player: Shape
}

impl GameRound {
    fn determine_player_score(&self) -> u32 {
        let outcome_score: u32 = match self.player_outcome() {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        };

        let shape_score = self.player.score();

        return shape_score + outcome_score;
    }

    fn player_outcome(&self) -> Outcome {
        match self.player {
            Shape::Rock => {
                match self.opponent {
                    Shape::Rock => Outcome::Draw,
                    Shape::Paper => Outcome::Loss,
                    Shape::Scissors => Outcome::Win,
                }
            }
            Shape::Paper => {
                match self.opponent {
                    Shape::Rock => Outcome::Win,
                    Shape::Paper => Outcome::Draw,
                    Shape::Scissors => Outcome::Loss,
                }
            }
            Shape::Scissors => {
                match self.opponent {
                    Shape::Rock => Outcome::Loss,
                    Shape::Paper => Outcome::Win,
                    Shape::Scissors => Outcome::Draw,
                }
            }
        }
    }

    fn shape_for_outcome(opponent_shape: &Shape, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Win => {
                match opponent_shape {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            }
            Outcome::Loss => {
                match opponent_shape {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            }
            Outcome::Draw => {
                // Tried to just return opponent_shape here but borrow checker complained
                match opponent_shape {
                    Shape::Rock => Shape::Rock,
                    Shape::Paper => Shape::Paper,
                    Shape::Scissors => Shape::Scissors,
                }
            }
        }
    }
}

fn main() {
    let file_handle = File::open("./src/input.txt").expect("Could not open file");
    let bufreader = BufReader::new(file_handle);

    let mut total_score: u32 = 0;

    for line in bufreader.lines() {
        let line = line.expect("Could not read line"); // Because iterator provides Result<String, Error>
        let split: Vec<&str> = line.split_whitespace().collect();
        let opponent_shape = match split[0] {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                    _ => panic!("Invalid input for opponent shape!")
        };
        let player_shape = match split[1] {
                    "X" => GameRound::shape_for_outcome(&opponent_shape, Outcome::Loss),
                    "Y" => GameRound::shape_for_outcome(&opponent_shape, Outcome::Draw),
                    "Z" => GameRound::shape_for_outcome(&opponent_shape, Outcome::Win),
                    _ => panic!("Invalid input for player shape!")
        };
        let round: GameRound = GameRound { opponent: opponent_shape, player: player_shape };

        total_score += round.determine_player_score();
    }

    println!("Player score: {total_score}");
}
