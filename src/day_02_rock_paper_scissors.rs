use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

#[derive(PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]
enum Result {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

struct UnexplainedGame {
    opponent: Play,
    response: Play,
}

struct ExplainedGame {
    opponent: Play,
    response: Result,
}

fn part1(games: Vec<UnexplainedGame>) -> i32 {
    let mut score = 0;
    for game in games {
        if (game.opponent == Play::Paper && game.response == Play::Rock)
            || (game.opponent == Play::Rock && game.response == Play::Scissors)
            || (game.opponent == Play::Scissors && game.response == Play::Paper)
        {
            score += game.response as i32;
            continue;
        }
        if game.opponent == game.response {
            score += 3 + game.response as i32;
            continue;
        }
        score += 6 + game.response as i32;
    }
    score
}

fn part2(games: Vec<ExplainedGame>) -> i32 {
    let mut score = 0;
    for game in games {
        if game.response == Result::Draw {
            score += 3 + game.opponent as i32;
        } else if game.response == Result::Lose {
            score += get_loose_play(game.opponent) as i32;
        } else {
            score += 6 + get_win_play(game.opponent) as i32;
        }
    }
    score
}

fn get_loose_play(input: Play) -> Play {
    let answer: Play = match input {
        Play::Paper => Play::Rock,
        Play::Rock => Play::Scissors,
        Play::Scissors => Play::Paper,
    };
    answer
}

fn get_win_play(input: Play) -> Play {
    let answer: Play = match input {
        Play::Paper => Play::Scissors,
        Play::Rock => Play::Paper,
        Play::Scissors => Play::Rock,
    };
    answer
}

fn transform_input_unexplained_games(input: &str) -> Vec<UnexplainedGame> {
    let lines: Vec<&str> = input.split('\n').filter(|&x| !x.is_empty()).collect();
    let input_transformed: Vec<UnexplainedGame> = lines
        .iter()
        .map(|&x| {
            let moves: Vec<&str> = x.split(' ').collect();
            let opponent: Play = match moves[0] {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                _ => Play::Scissors,
            };
            let response: Play = match moves[1] {
                "X" => Play::Rock,
                "Y" => Play::Paper,
                "Z" => Play::Scissors,
                _ => Play::Scissors,
            };
            UnexplainedGame { opponent, response }
        })
        .collect();
    input_transformed
}

fn transform_input_explained_games(input: &str) -> Vec<ExplainedGame> {
    let lines: Vec<&str> = input.split('\n').filter(|&x| !x.is_empty()).collect();
    let input_transformed: Vec<ExplainedGame> = lines
        .iter()
        .map(|&x| {
            let moves: Vec<&str> = x.split(' ').collect();
            let opponent: Play = match moves[0] {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                _ => Play::Scissors,
            };
            let response: Result = match moves[1] {
                "X" => Result::Lose,
                "Y" => Result::Draw,
                "Z" => Result::Win,
                _ => Result::Win,
            };
            ExplainedGame { opponent, response }
        })
        .collect();
    input_transformed
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("src/day_02_input.txt");
    let result_1 = part1(transform_input_unexplained_games(&input));
    let result_2 = part2(transform_input_explained_games(&input));
    (Solution::I32(result_1), Solution::I32(result_2))
}
