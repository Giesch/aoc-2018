use std::str;

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_5_input.txt");

    let solution_one = part_one(input)?;
    println!("Part One: {}", solution_one);

    // let solution_two = part_two(input)?;
    // println!("Part Two: {}", solution_two);

    Ok(())
}

fn part_one(input: &str) -> StdResult<usize> {
    let result = react(input)?;
    Ok(result.len())
}

fn react(input: &str) -> Result<String, str::Utf8Error> {
    let mut bytes: Vec<_> = input.as_bytes().to_vec();

    while let Some(i) = find_reaction(&bytes) {
        bytes.remove(i);
        bytes.remove(i);
    }

    let result = str::from_utf8(&bytes)?;
    Ok(result.to_string())
}

fn find_reaction(bytes: &[u8]) -> Option<usize> {
    bytes
        .windows(2)
        .enumerate()
        .find(|(_, slice)| match slice {
            [a, b] => should_react(*a, *b),
            _ => panic!(),
        })
        .map(|(i, _)| i)
}

fn should_react(a: u8, b: u8) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part_one_example() {
        let result = part_one(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn react_test() {
        let result = react(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "dabCBAcaDA");
    }

    #[test]
    fn find_reaction_test() {
        let input = EXAMPLE_INPUT.as_bytes();
        let i = find_reaction(input).unwrap();
        assert_eq!(i, 4);
    }

    #[test]
    fn should_react_test() {
        let result = match "yZ".as_bytes() {
            [a, b] => should_react(*a, *b),
            _ => panic!(),
        };
        assert!(!result);

        let result = match "Aa".as_bytes() {
            [a, b] => should_react(*a, *b),
            _ => panic!(),
        };

        assert!(result);
        let result = match "zZ".as_bytes() {
            [a, b] => should_react(*a, *b),
            _ => panic!(),
        };
        assert!(result);

        let result = match "zz".as_bytes() {
            [a, b] => should_react(*a, *b),
            _ => panic!(),
        };
        assert!(!result);
    }
}
