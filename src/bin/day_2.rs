use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

fn main() {
    let input = include_str!("../day_2_input.txt");

    let solution_one = part_one(input);
    println!("Part One: {}", solution_one);

    let solution_two = part_two(input);
    println!("Part Two: {}", solution_two);
}

fn counts(id: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    counts
}

fn contains_count(id: &str, count: usize) -> bool {
    counts(id).values().any(|&c| c == count)
}

fn checksum_counts(id: &str) -> (usize, usize) {
    let two = if contains_count(id, 2) { 1 } else { 0 };
    let three = if contains_count(id, 3) { 1 } else { 0 };

    (two, three)
}

fn part_one(input: &str) -> usize {
    let (twos, threes) = input
        .lines()
        .map(checksum_counts)
        .fold((0, 0), |(twos, threes), (two, three)| {
            (twos + two, threes + three)
        });

    twos * threes
}

fn count_differences(left: &str, right: &str) -> u32 {
    left.chars()
        .zip(right.chars())
        .fold(0, |d, (l, r)| if l == r { d } else { d + 1 })
}

fn correct_boxes(left: &str, right: &str) -> bool {
    count_differences(left, right) == 1
}

fn matching_boxes(input: &str) -> Option<(String, String)> {
    let mut seen: HashSet<String> = HashSet::new();
    for line in input.lines() {
        for id in &seen {
            if correct_boxes(id, line) {
                return Some((id.to_string(), line.to_string()));
            }
        }

        seen.insert(line.to_string());
    }

    None
}

fn part_two(input: &str) -> String {
    let (left, right) = matching_boxes(input).unwrap();

    left.chars()
        .zip(right.chars())
        .fold(String::new(), |mut solution, (l, r)| {
            if l == r {
                solution.push(l);
            }

            solution
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!("fgij", part_two(input));
    }

    #[test]
    fn correct_boxes_test() {
        assert!(correct_boxes("fghij", "fguij"));
    }
}
