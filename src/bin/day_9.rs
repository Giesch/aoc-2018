#![feature(vecdeque_rotate)]

use std::collections::HashMap;
use std::collections::VecDeque;

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let solution_one = part_one(400, 71_864);
    println!("Part One: {}", solution_one);

    let solution_two = part_two(400, 7_186_400);
    println!("Part Two: {}", solution_two);

    Ok(())
}

fn part_one(players: usize, last_marble_val: usize) -> usize {
    let mut current = 0;
    let mut circle = vec![0];
    let mut scores = HashMap::new();

    for turn in 1..=last_marble_val {
        let player = turn % players;
        let (new_current, new_circle, score) = add_marble(current, circle, turn);
        current = new_current;
        circle = new_circle;
        let player_score = scores.entry(player).or_insert(0);
        *player_score += score;
    }

    *scores.values().max().unwrap()
}

fn part_two(players: usize, last_marble_val: usize) -> usize {
    let mut circle = VecDeque::new();
    circle.push_front(0);
    let mut scores = HashMap::new();

    for turn in 1..=last_marble_val {
        let (new_circle, score) = new_add_marble(circle, turn);
        circle = new_circle;

        let player = turn % players;
        let player_score = scores.entry(player).or_insert(0);
        *player_score += score;
    }

    *scores.values().max().unwrap()
}

fn new_add_marble(mut circle: VecDeque<usize>, next_marble: usize) -> (VecDeque<usize>, usize) {
    if next_marble % 23 == 0 {
        let mut score = next_marble;
        circle.rotate_right(7);
        score += circle.pop_front().unwrap();
        return (circle, score);
    }

    circle.rotate_left(1);
    circle.rotate_left(1);
    circle.push_front(next_marble);
    (circle, 0)
}

fn add_marble(
    current: usize,
    mut circle: Vec<usize>,
    next_marble: usize,
) -> (usize, Vec<usize>, usize) {
    if next_marble % 23 == 0 {
        let mut score = next_marble;
        let other_marble = ((current + circle.len()) - 7) % circle.len();
        score += circle.remove(other_marble);
        let new_current = other_marble % circle.len();
        return (new_current, circle, score);
    }

    let position = (current + 2) % circle.len();
    circle.insert(position, next_marble);

    (position, circle, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_circle() -> (usize, Vec<usize>) {
        (0, vec![0])
    }

    #[test]
    fn add_marble_test() {
        let (current, circle) = new_circle();
        let (current, circle, _score) = add_marble(current, circle, 1);
        assert_eq!(circle[current], 1);

        let (current, circle, _score) = add_marble(current, circle, 2);
        assert_eq!(circle[current], 2);

        let (current, circle, _score) = add_marble(current, circle, 3);
        assert_eq!(circle[current], 3);

        assert_eq!(circle, vec![2, 1, 3, 0]);
    }

    #[test]
    fn add_22_marbles() {
        let (current, circle) = new_circle();
        let mut current = current;
        let mut circle = circle;
        for turn in 1..=22 {
            let (new_current, new_circle, score) = add_marble(current, circle, turn);
            assert_eq!(score, 0);
            current = new_current;
            circle = new_circle;
        }

        let expected_circle = vec![
            16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0,
        ];

        assert_eq!(expected_circle, circle);
    }

    #[test]
    fn add_23_marbles() {
        let (current, circle) = new_circle();
        let mut current = current;
        let mut circle = circle;
        for turn in 1..=22 {
            let (new_current, new_circle, score) = add_marble(current, circle, turn);
            assert_eq!(score, 0);
            current = new_current;
            circle = new_circle;
        }

        let (_current, circle, score) = add_marble(current, circle, 23);
        assert_eq!(score, 32);

        let expected_circle = vec![
            16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0,
        ];

        assert_eq!(expected_circle, circle);
    }

    #[test]
    fn part_one_examples() {
        let result = part_one(9, 25);
        assert_eq!(result, 32);

        let result = part_one(10, 1618);
        assert_eq!(result, 8317);

        let result = part_one(13, 7999);
        assert_eq!(result, 146373);
    }

    #[test]
    fn part_two_examples() {
        let result = part_two(9, 25);
        assert_eq!(result, 32);

        let result = part_two(10, 1618);
        assert_eq!(result, 8317);

        let result = part_two(13, 7999);
        assert_eq!(result, 146373);
    }

}
