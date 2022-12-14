#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scisors,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum RoundOutcome {
    Loss,
    Draw,
    Win,
}

impl RoundOutcome {
    fn new(round: GameRound) -> Self {
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

#[derive(Debug, Clone)]
struct GameRound {
    opponent_move: Move,
    player_move: Move,
}

impl GameRound {
    fn new(opponent_move: Move, player_move: Move) -> Self {
        Self {
            opponent_move,
            player_move,
        }
    }
}

fn parse_line(line: &str) -> (Move, Move) {
    let mut line = line.split_whitespace();
    let opponent_move = line.next().unwrap_or("");
    let player_move = line.next().unwrap_or("");

    let opponent_move = match opponent_move {
        "A" => Move::Rock,
        "B" => Move::Paper,
        _ => Move::Scisors,
    };

    let player_move = match player_move {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        _ => Move::Scisors,
    };

    (opponent_move, player_move)
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<GameRound> {
    input
        .lines()
        .map(parse_line)
        .map(|(opponent_move, player_move)| GameRound::new(opponent_move, player_move))
        .collect()
}

fn score_round(round: &GameRound) -> u32 {
    let player_move_score = match round.player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        _ => 3,
    };

    let outcome = RoundOutcome::new(round.to_owned());

    let round_outcome_score = match outcome {
        RoundOutcome::Loss => 0,
        RoundOutcome::Draw => 3,
        RoundOutcome::Win => 6,
    };

    player_move_score + round_outcome_score
}

#[aoc(day2, part1)]
fn rock_paper_scisors(input: &[GameRound]) -> u32 {
    input.iter().map(score_round).sum()
}
