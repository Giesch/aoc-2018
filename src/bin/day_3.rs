use std::collections::{HashMap, HashSet};

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_3_input.txt");

    let solution_one = part_one(input)?;
    println!("Part One: {}", solution_one);

    let solution_two = part_two(input)?;
    println!("Part Two: {}", solution_two);

    Ok(())
}

#[derive(Default)]
struct Claim {
    id: i32,
    corner_x: i32,
    corner_y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn parse(input: &str) -> StdResult<Claim> {
        let v: Vec<&str> = input.split_whitespace().collect();

        let mut claim = Claim::default();

        match v.as_slice() {
            [id, _, origin, size] => {
                let mut id = id.to_string();
                id.remove(0); // # sign
                claim.id = id.parse()?;

                let o: Vec<&str> = origin.split(",").collect();
                match o.as_slice() {
                    [x, y] => {
                        let mut y = y.to_string();
                        y.pop(); // get rid of colon
                        claim.corner_x = x.parse()?;
                        claim.corner_y = y.parse()?;
                    }
                    _ => Err("parse corner failed")?,
                }

                let s: Vec<&str> = size.split("x").collect();
                match s.as_slice() {
                    [w, h] => {
                        claim.width = w.parse()?;
                        claim.height = h.parse()?;
                    }
                    _ => Err("parse size failed")?,
                }

                Ok(claim)
            }
            _ => Err("parse claim failed")?,
        }
    }

    fn indicies(&self) -> HashSet<(i32, i32)> {
        let mut indicies = HashSet::new();
        for x in self.corner_x..(self.corner_x + self.width) {
            for y in self.corner_y..(self.corner_y + self.height) {
                indicies.insert((x, y));
            }
        }

        indicies
    }
}

fn parse_claims(input: &str) -> StdResult<Vec<Claim>> {
    input.lines().map(Claim::parse).collect()
}

fn fabric_map(claims: &Vec<Claim>) -> HashMap<(i32, i32), i32> {
    let mut fabric = HashMap::new();
    for claim in claims {
        for i in claim.indicies() {
            let e = fabric.entry(i).or_insert(0);
            *e += 1;
        }
    }

    fabric
}

fn part_one(input: &str) -> StdResult<usize> {
    let claims = parse_claims(input)?;
    let fabric = fabric_map(&claims);

    Ok(fabric.values().filter(|&&v| v > 1).count())
}

fn part_two(input: &str) -> StdResult<i32> {
    let claims = parse_claims(input)?;
    let fabric = fabric_map(&claims);

    let solution = claims
        .iter()
        .find(|claim| {
            claim
                .indicies()
                .iter()
                .all(|&i| fabric.get(&i).unwrap_or(&0) == &1)
        })
        .ok_or("no 'intact' claim found")?;

    Ok(solution.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "#1 @ 1,3: 4x4";
        let claim = Claim::parse(input).unwrap();
        assert_eq!(claim.id, 1);
        assert_eq!(claim.corner_x, 1);
        assert_eq!(claim.corner_y, 3);
        assert_eq!(claim.width, 4);
        assert_eq!(claim.height, 4);
    }

    #[test]
    fn part_one_example() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(4, part_one(input).unwrap());
    }

    #[test]
    fn part_two_example() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(3, part_two(input).unwrap());
    }
}
