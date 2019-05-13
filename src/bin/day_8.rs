type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_8_input.txt").trim();

    let solution_one = part_one(input)?;
    println!("Part One: {}", solution_one);

    let solution_two = part_two(input)?;
    println!("Part Two: {}", solution_two);

    Ok(())
}

fn part_one(input: &str) -> StdResult<usize> {
    let node = parse_input(input)?;
    let sum = sum_metadata(node);
    Ok(sum)
}

fn part_two(input: &str) -> StdResult<usize> {
    let node = parse_input(input)?;
    let value = value(&node);
    Ok(value)
}

fn value(node: &Node) -> usize {
    if node.children.is_empty() {
        return node.metadata.iter().sum();
    }

    let indicies: Vec<_> = node
        .metadata
        .iter()
        .map(|i| i - 1)
        .filter(|&i| i < node.children.len())
        .collect();

    let mut val = 0;
    for index in indicies {
        val += value(&node.children[index])
    }

    val
}

fn sum_metadata(node: Node) -> usize {
    let mut sum = node.metadata.iter().sum();
    for child in node.children {
        sum += sum_metadata(child);
    }

    sum
}

struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}

fn parse_input(input: &str) -> StdResult<Node> {
    let nums = parse_numbers(input)?;
    let (node, remaining) = parse_node(nums);

    if !remaining.is_empty() {
        Err("remaining numbers")?
    }

    Ok(node)
}

fn parse_numbers(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input.split_whitespace().map(str::parse::<usize>).collect()
}

fn parse_node(mut v: Vec<usize>) -> (Node, Vec<usize>) {
    let child_count = v[0];
    let meta_count = v[1];
    v.drain(0..2);

    let mut children = vec![];
    for _child in 0..child_count {
        let (child, remaining) = parse_node(v);
        v = remaining;
        children.push(child);
    }

    let metadata: Vec<_> = v.drain(0..meta_count).collect();

    let node = Node { metadata, children };

    (node, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part_one_test() {
        let result = part_one(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 138);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 66);
    }

}
