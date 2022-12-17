#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scisors,
}

impl Move {
    fn new(game_move: &str) -> Self {
        match game_move {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scisors,
            _ => Move::Scisors,
        }
    }
}

struct GameOne {
    opponent_move: Move,
    player_move: Move,
}

impl From<&Input> for GameOne {
    fn from(input: &Input) -> Self {
        let opponent_move = Move::new(&input.left);
        let player_move = Move::new(&input.right);

        GameOne {opponent_move, player_move }
    }
}

#[derive(Debug)]
enum RoundOutcome {
    Loss,
    Draw,
    Win,
}

impl RoundOutcome {
    fn new(round: GameOne) -> RoundOutcome {
        match (round.opponent_move, round.player_move) {
            (Move::Rock, Move::Paper) => RoundOutcome::Win,
            (Move::Rock, Move::Scisors) => RoundOutcome::Loss,
            (Move::Paper, Move::Scisors) => RoundOutcome::Win,
            (Move::Paper, Move::Rock) => RoundOutcome::Loss,
            (Move::Scisors, Move::Rock) => RoundOutcome::Win,
            (Move::Scisors, Move::Paper) => RoundOutcome::Loss,
            _ => RoundOutcome::Draw,
        }
    }
}

struct Input {
    left: String,
    right: String,
}

impl Input {
    fn new(left: &str, right: &str) -> Self {
        Self { left: left.into(), right: right.into() }
    }
}

fn parse_line(line: &str) -> Input {
    let mut line = line.split_whitespace();
    let left = line.next().unwrap_or("");
    let right = line.next().unwrap_or("");
    Input::new(left, right)
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(parse_line)
        .collect()
}

fn score_round(round: GameOne) -> u32 {
    let player_move_score = match round.player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        _ => 3,
    };

    let outcome = RoundOutcome::new(round);

    let round_outcome_score = match outcome {
        RoundOutcome::Loss => 0,
        RoundOutcome::Draw => 3,
        RoundOutcome::Win => 6,
    };

    player_move_score + round_outcome_score
}

#[aoc(day2, part1)] 
fn game_one(input: &[Input]) -> u32 {
    input 
        .iter()
        .map(|input| GameOne::from(input))
        .map(score_round)
        .sum()
}
