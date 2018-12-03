use std::collections::HashSet;

fn main() {
    let input = include_str!("../day_1_input.txt");

    let vals: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();

    let one: i64 = vals.iter().sum();
    println!("Part One: {}", one);

    let two: i64 = part_two(vals);
    println!("Part Two: {}", two);
}

fn part_two(vals: Vec<i64>) -> i64 {
    let mut seen = HashSet::new();
    seen.insert(0);

    vals.iter()
        .cycle()
        .scan(0, |freq, &item| {
            *freq += item;
            Some(*freq)
        })
        .filter(|&f| {
            if seen.contains(&f) {
                true
            } else {
                seen.insert(f);
                false
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(0, part_two(vec![1, -1]));
    }

    #[test]
    fn example_two() {
        assert_eq!(10, part_two(vec![3, 3, 4, -2, -4]));
    }
}
