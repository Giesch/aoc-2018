use std::collections::HashMap;
use std::collections::HashSet;

type StdResult<T> = Result<T, Box<std::error::Error>>;

type Coord = (i32, i32);

// The usize is a region id for what the coordinate is closest to
type Grid = HashMap<Coord, Option<usize>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_6_input.txt").trim();

    let solution_one = part_one(input)?;
    println!("Part One: {}", solution_one);

    let solution_two = part_two(input)?;
    println!("Part Two: {}", solution_two);

    Ok(())
}

fn part_two(input: &str) -> StdResult<usize> {
    let coords = parse_coordinates(input)?;
    let grid = count_distances(coords)?;
    let sol = grid.values().filter(|&&count| count < 10_000).count();
    Ok(sol)
}

fn count_distances(centers: Vec<Coord>) -> StdResult<HashMap<Coord, usize>> {
    let (x_max, y_max) = maximums(&centers)?;
    let mut result = HashMap::new();

    for x in 0..=x_max {
        for y in 0..=y_max {
            let coordinate = (x, y);
            for center in &centers {
                let distance = manhattan(coordinate, *center);
                let total = result.entry(coordinate).or_insert(0);
                *total += distance;
            }
        }
    }

    Ok(result)
}

fn manhattan(a: Coord, b: Coord) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn part_one(input: &str) -> StdResult<usize> {
    let coords = parse_coordinates(input)?;
    let num_regions = coords.len();
    let maxes = maximums(&coords)?;

    let mut grid = grid(coords);
    let mut done = false;

    while !done {
        let (done_now, new_grid) = djikstra_round(grid, maxes);
        done = done_now;
        grid = new_grid;
    }

    let edge_regions = edge_regions(&grid, maxes);

    let mut max = 0;
    for region_id in 0..num_regions {
        if !edge_regions.contains(&region_id) {
            let count = count_grid(&grid, region_id);
            if count > max {
                max = count;
            }
        }
    }

    Ok(max)
}

fn edge_regions(grid: &Grid, maxes: Coord) -> HashSet<usize> {
    let mut regions = HashSet::new();
    let (x_max, y_max) = maxes;

    for x in 0..=x_max {
        if let Some(&Some(region)) = grid.get(&(x, 0)) {
            regions.insert(region);
        }
        if let Some(&Some(region)) = grid.get(&(x, y_max)) {
            regions.insert(region);
        }
    }

    for y in 0..=y_max {
        if let Some(&Some(region)) = grid.get(&(0, y)) {
            regions.insert(region);
        }
        if let Some(&Some(region)) = grid.get(&(x_max, y)) {
            regions.insert(region);
        }
    }

    regions
}

fn count_grid(grid: &Grid, region_id: usize) -> usize {
    grid.values()
        .filter_map(|region| *region)
        .filter(|&id| id == region_id)
        .count()
}

fn djikstra_round(mut grid: Grid, maxes: Coord) -> (bool, Grid) {
    let (x_max, y_max) = maxes;
    let mut done = true;

    // usize here is group id; None = tie between regions
    // do we care about what regions they are? or their distance? not for part one
    let mut marks: HashMap<Coord, HashSet<usize>> = HashMap::new();

    for x in 0..=x_max {
        for y in 0..=y_max {
            let neighboring_regions: HashSet<usize> = neighbors((x, y))
                .iter()
                .filter_map(|coord| grid.get(coord))
                .filter_map(|region| *region)
                .collect();

            if neighboring_regions.is_empty() {
                done = false;
            } else {
                if let None = grid.get(&(x, y)) {
                    marks.insert((x, y), neighboring_regions);
                }
            }
        }
    }

    for (coord, regions) in marks {
        if regions.len() > 1 {
            grid.insert(coord, None);
        } else if regions.len() == 1 {
            for region in regions {
                grid.insert(coord, Some(region));
            }
        }
    }

    (done, grid)
}

fn parse_coordinates(input: &str) -> StdResult<Vec<Coord>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> StdResult<Coord> {
    let s: Vec<_> = line.split(", ").collect();
    match s.as_slice() {
        [x, y] => Ok((x.parse()?, y.parse()?)),
        _ => Err("parsing failed")?,
    }
}

fn maximums(v: &Vec<Coord>) -> StdResult<Coord> {
    let &x = v.iter().map(|(x, _)| x).max().ok_or("empty data")?;
    let &y = v.iter().map(|(_, y)| y).max().ok_or("empty data")?;
    Ok((x, y))
}

fn grid(coordinates: Vec<Coord>) -> Grid {
    coordinates
        .iter()
        .enumerate()
        .map(|(i, coord)| (*coord, Some(i)))
        .collect()
}

fn neighbors(coordinate: Coord) -> [Coord; 4] {
    let (x, y) = coordinate;
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";

    #[test]
    fn part_one_example() {
        let result = part_one(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 17)
    }

    #[test]
    fn parsing() {
        let v = parse_coordinates(EXAMPLE_INPUT).unwrap();
        assert_eq!(v[0], (1, 1))
    }

}
