#![feature(vecdeque_rotate)]

use std::collections::HashMap;
use std::collections::VecDeque;

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let solution_one = solve(400, 71_864);
    println!("Part One: {}", solution_one);

    let solution_two = solve(400, 7_186_400);
    println!("Part Two: {}", solution_two);

    Ok(())
}

struct Turn {
    circle: VecDeque<usize>,
    marble: usize,
}

impl Turn {
    fn new(circle: VecDeque<usize>, marble: usize) -> Turn {
        Turn { circle, marble }
    }
}

struct TurnResult {
    circle: VecDeque<usize>,
    score: usize,
}

impl TurnResult {
    fn new(circle: VecDeque<usize>, score: usize) -> TurnResult {
        TurnResult { circle, score }
    }
}

fn solve(players: usize, last_marble: usize) -> usize {
    let mut circle = VecDeque::new();
    circle.push_front(0);
    let mut scores = HashMap::new();

    for turn in 1..=last_marble {
        let result = add_marble(Turn::new(circle, turn));
        circle = result.circle;
        let player_score = scores.entry(turn % players).or_insert(0);
        *player_score += result.score;
    }

    *scores.values().max().unwrap()
}

fn add_marble(turn: Turn) -> TurnResult {
    let Turn { mut circle, marble } = turn;

    if marble % 23 == 0 {
        let mut score = marble;
        circle.rotate_right(7);
        score += circle.pop_front().unwrap();
        return TurnResult::new(circle, score);
    }

    circle.rotate_left(1);
    circle.rotate_left(1);
    circle.push_front(turn.marble);
    TurnResult::new(circle, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let result = solve(9, 25);
        assert_eq!(result, 32);

        let result = solve(10, 1618);
        assert_eq!(result, 8317);

        let result = solve(13, 7999);
        assert_eq!(result, 146373);
    }

    #[test]
    fn part_one_answer() {
        let solution_one = solve(400, 71_864);
        assert_eq!(solution_one, 437654);
    }

    #[test]
    #[ignore]
    fn part_two_answer() {
        let solution_two = solve(400, 7_186_400);
        assert_eq!(solution_two, 3689913905);
    }

}
