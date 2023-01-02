use std::{fs, fmt::Display};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Round {
    opponent_score: u8,
    player_score: u8
}

impl Round {
    const ROUND_WIN_SCORE: u8 = 6;
    const ROUND_DRAW_SCORE: u8 = 3;
    const ROUND_LOSE_SCORE: u8 = 0;

    fn new (opponent_score: u8, player_score: u8) -> Round {
        return Round { opponent_score, player_score}
    }
}

fn main() -> Result<()> {
    let mut opponent_score: u32 = 0;
    let mut player_score: u32 = 0;

    let rounds = fs::read_to_string("input_rpc.txt")?;
    for round in rounds.lines() {
        let round = match get_players_hands(round) {
            Ok((Hand::Rock, Outcome::Lose)) => 
                Round::new(Round::ROUND_WIN_SCORE + Hand::Rock as u8, Round::ROUND_LOSE_SCORE + Hand::Scissors as u8),
            Ok((Hand::Paper, Outcome::Draw)) => 
                Round::new(Round::ROUND_DRAW_SCORE + Hand::Paper as u8, Round::ROUND_DRAW_SCORE + Hand::Paper as u8),
            Ok((Hand::Scissors, Outcome::Win)) => 
                Round::new(Round::ROUND_LOSE_SCORE + Hand::Scissors as u8, Round::ROUND_WIN_SCORE + Hand::Rock as u8),
            Ok((Hand::Rock, Outcome::Draw)) => 
                Round::new(Round::ROUND_DRAW_SCORE + Hand::Rock as u8, Round::ROUND_DRAW_SCORE + Hand::Rock as u8),
            Ok((Hand::Paper, Outcome::Win)) => 
                Round::new(Round::ROUND_LOSE_SCORE + Hand::Paper as u8, Round::ROUND_WIN_SCORE + Hand::Scissors as u8),
            Ok((Hand::Scissors, Outcome::Lose)) => 
                Round::new(Round::ROUND_WIN_SCORE + Hand::Scissors as u8, Round::ROUND_LOSE_SCORE + Hand::Paper as u8),
            Ok((Hand::Rock, Outcome::Win)) => 
                Round::new(Round::ROUND_LOSE_SCORE + Hand::Rock as u8, Round::ROUND_WIN_SCORE + Hand::Paper as u8),
            Ok((Hand::Paper, Outcome::Lose)) => 
                Round::new(Round::ROUND_WIN_SCORE + Hand::Paper as u8, Round::ROUND_LOSE_SCORE + Hand::Rock as u8),
            Ok((Hand::Scissors, Outcome::Draw)) => 
                Round::new(Round::ROUND_DRAW_SCORE + Hand::Scissors as u8, Round::ROUND_DRAW_SCORE + Hand::Scissors as u8),
            Err(error_type) => {
                println!("{}", error_type);
                Round::new(0, 0)
            }
        };

        opponent_score += &(round.opponent_score as u32);
        player_score += &(round.player_score as u32);
    }

    println!("Opponent score: {}", opponent_score);
    println!("Player score: {}", player_score);

    return Ok(());
}

fn get_players_hands(turns_line: &str) -> Result<(Hand, Outcome)> {
    if turns_line.len() != 3 {
        return Err(Box::from(ParseErrors::TurnIncorrectFormat));
    }

    let opponent_turn = match turns_line.chars().nth(0) {
        Some('A') => Ok(Hand::Rock),
        Some('B') => Ok(Hand::Paper),
        Some('C') => Ok(Hand::Scissors),
        Some(x) => Err(Box::new(ParseErrors::OpponentHandIncorrect(x))),
        None => Err(Box::from(ParseErrors::OpponentHandIncorrect(' ')))
    }?;

    let outcome = match turns_line.chars().nth(2) {
        Some('X') => Ok(Outcome::Lose),
        Some('Y') => Ok(Outcome::Draw),
        Some('Z') => Ok(Outcome::Win),
        Some(x) => Err(Box::new(ParseErrors::MyHandIncorrect(x))),
        None => Err(Box::from(ParseErrors::MyHandIncorrect(' ')))
    }?;

    return Ok((opponent_turn, outcome));
}

#[derive(Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose = 1,
    Draw = 2,
    Win = 3
}

#[derive(Debug)]
enum ParseErrors {
    TurnIncorrectFormat,
    OpponentHandIncorrect(char),
    MyHandIncorrect(char)
}

impl std::error::Error for ParseErrors {}
impl Display for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseErrors::TurnIncorrectFormat => {
                write!(f, "Turn format was not parsed, because it was not correct.")
            }
            ParseErrors::OpponentHandIncorrect(turn_value) => write!(
                f, "Invalid opponent turn value '{}', possible values 'A', 'B', 'C'", turn_value
            ),
            ParseErrors::MyHandIncorrect(turn_value)=> write!(
                f, "Invalid my turn value '{}', possible values 'X', 'Y', 'Z'", turn_value
            ),
        }
    }
}

/*
Exercise:

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/