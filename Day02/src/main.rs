use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

/// What a player can show in their hand each play
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum HandOption {
    Rock,
    Paper,
    Scissors,
}
impl HandOption {
    fn hand_score(&self) -> i32 {
        return match self{
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
    }
}

impl FromStr for HandOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "A" => Ok(HandOption::Rock),
            "B" => Ok(HandOption::Paper),
            "C" => Ok(HandOption::Scissors),
            "X" => Ok(HandOption::Rock),
            "Y" => Ok(HandOption::Paper),
            "Z" => Ok(HandOption::Scissors),
            _ => Err(format!("{s} is not a valid input for the strategy!")),
        }
    }
}

/// One play, first vs second
#[derive(Debug, Clone)]
struct Play {
    first: HandOption,
    second: HandOption,
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum PlayResult{
    Win,
    Lose,
    Draw,
}

impl Play{

    /// Score for the second player
    pub fn score(&self) -> i32 {
        let hand_score = self.second.hand_score();
        let play_result = self.result_for_second();

        let play_score = match play_result {
            PlayResult::Lose => 0,
            PlayResult::Draw => 3,
            PlayResult::Win => 6,
        };

        return hand_score + play_score;
    }

    fn result_for_second(&self) -> PlayResult {
        let mut result_table = HashMap::new();

        result_table.insert((HandOption::Rock, HandOption::Rock), PlayResult::Draw);
        result_table.insert((HandOption::Rock, HandOption::Paper), PlayResult::Win);
        result_table.insert((HandOption::Rock, HandOption::Scissors), PlayResult::Lose);

        result_table.insert((HandOption::Paper, HandOption::Rock), PlayResult::Lose);
        result_table.insert((HandOption::Paper, HandOption::Paper), PlayResult::Draw);
        result_table.insert((HandOption::Paper, HandOption::Scissors), PlayResult::Win);

        result_table.insert((HandOption::Scissors, HandOption::Rock), PlayResult::Win);
        result_table.insert((HandOption::Scissors, HandOption::Paper), PlayResult::Lose);
        result_table.insert((HandOption::Scissors, HandOption::Scissors), PlayResult::Draw);

        let result = result_table.get(&(self.first.clone(), self.second.clone())).expect("Some variation is not covered in the results table");
        return result.clone();
    }

    /// Fix a play so the second player uses the optimal play
    fn fix(&self) -> Play {
        let mut result_table = HashMap::new();

        let we_want_to_get = match self.second {
            HandOption::Rock => PlayResult::Lose,
            HandOption::Paper => PlayResult::Draw,
            HandOption::Scissors => PlayResult::Win,
        };

        result_table.insert((HandOption::Rock, PlayResult::Draw), HandOption::Rock);
        result_table.insert((HandOption::Rock, PlayResult::Lose), HandOption::Scissors);
        result_table.insert((HandOption::Rock, PlayResult::Win), HandOption::Paper);

        result_table.insert((HandOption::Paper, PlayResult::Draw), HandOption::Paper);
        result_table.insert((HandOption::Paper, PlayResult::Lose), HandOption::Rock);
        result_table.insert((HandOption::Paper, PlayResult::Win), HandOption::Scissors);

        result_table.insert((HandOption::Scissors, PlayResult::Draw), HandOption::Scissors);
        result_table.insert((HandOption::Scissors, PlayResult::Lose), HandOption::Paper);
        result_table.insert((HandOption::Scissors, PlayResult::Win), HandOption::Rock);


        let result = result_table.get(&(self.first.clone(), we_want_to_get)).expect("Some variation is not covered in the results table");
        let result = result.clone();

        return Play{first: self.first.clone(), second: result};
    }
}

/// Represents a whole game
#[derive(Debug)]
struct Game {
    plays: Vec<Play>
}

impl Game {
    pub fn score(&self) -> i32{
        return self.plays.iter().map(|play| play.score()).sum();
    }

    /// Fixes a game so the second player actually uses the optimal strategy
    fn fix_game(&self) -> Self {
        let mut plays = vec![];

        for play in &self.plays {
            let fixed_play = play.fix();
            plays.push(fixed_play);
        }

        return Game{plays};
    }
}

// Given a strategy file, returns the obtained score
fn strategy_score(strategy_file: &str) -> i32 {
    let game = parse_file(strategy_file);
    return game.score();
}

// Given a strategy file, returns the obtained score with the optimal strategy (part two)
// The score is computed in the same way, we just need to change the HandOption of the second player
fn optimal_strategy_score(strategy_file: &str) -> i32 {
    let game = parse_file(strategy_file);
    let game = game.fix_game(); // This is what we add to use the optimal strat
    return game.score();
}

fn parse_file(file_path: &str) -> Game {
    let file = File::open(file_path).expect("Error opening given file");
    let lines = io::BufReader::new(file).lines();

    let mut plays = vec![];

    for line in lines {
        let line = line.expect("Failed reading one line of the file");
        let pair: Vec<&str> = line.split(" ").collect();

        // Security check
        if pair.len() != 2 {
            let err_msg = format!("Current line has {} elements, expected 2!", pair.len());
            panic!("{err_msg}");
        }

        let first: HandOption = pair[0].parse().unwrap();
        let second: HandOption = pair[1].parse().unwrap();

        let play = Play{first, second};
        plays.push(play);
    }

    return Game{plays};
}

fn main() {
    let strategy_file = "./data/input.txt";
    let score = strategy_score(strategy_file);
    println!("Part one score is {score}");

    let score = optimal_strategy_score(strategy_file);
    println!("Part two score is {score}");
}

#[cfg(test)]
mod tests {
    use crate::{strategy_score, optimal_strategy_score};

    #[test]
    pub fn test_given_example_part_one() {
        let strategy_file = "./data/input_example.txt";

        let expected = 15;
        let computed = strategy_score(strategy_file);

        assert_eq!(expected, computed, "Score is not the same as the given in the example");

    }

    #[test]
    pub fn test_given_example_part_two() {
        let strategy_file = "./data/input_example.txt";

        let expected = 12;
        let computed = optimal_strategy_score(strategy_file);

        assert_eq!(expected, computed, "Score is not the same as the given in the example");

    }
}
