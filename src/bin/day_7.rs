use std::collections::HashMap;
use std::collections::HashSet;

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_7_input.txt").trim();

    let tasks: HashSet<u8> = (b'A'..=b'Z').collect();
    let solution_one = part_one(input, &tasks)?;
    println!("Part One: {}", solution_one);

    let solution_two = part_two(input, &tasks, 60, 5)?;
    println!("Part Two: {}", solution_two);

    Ok(())
}

type Dependencies = HashMap<u8, HashSet<u8>>;

fn part_one(input: &str, tasks: &HashSet<u8>) -> StdResult<String> {
    let deps_map = parse_dependencies(input)?;
    let mut result = "".to_string();

    let mut completed_tasks: HashSet<u8> = HashSet::new();
    while completed_tasks.len() < tasks.len() {
        let available_tasks = available_tasks(tasks, &deps_map, &completed_tasks);

        let &next = available_tasks.iter().min().expect("no available tasks");
        result.push(next as char);
        completed_tasks.insert(next);
    }

    Ok(result)
}

fn part_two(input: &str, tasks: &HashSet<u8>, wait: i32, workers: usize) -> StdResult<usize> {
    let deps_map = parse_dependencies(input)?;

    let mut time_taken = 0;
    let mut workers: Vec<Option<u8>> = vec![None; workers];
    let mut completed_tasks: HashSet<u8> = HashSet::new();
    let mut work_remaining: HashMap<u8, i32> = (b'A'..=b'Z')
        .enumerate()
        .map(|(i, task)| (task, wait + 1 + i as i32))
        .collect();

    while completed_tasks != *tasks {
        time_taken += 1;
        let available_tasks = available_tasks(&tasks, &deps_map, &completed_tasks);
        workers = assign_workers(workers, available_tasks);

        for worker in 0..workers.len() {
            if let Some(task) = workers[worker] {
                let time = work_remaining.entry(task).or_insert_with(|| panic!("agh"));
                *time -= 1;

                if *time <= 0 {
                    workers[worker] = None;
                    completed_tasks.insert(task);
                }
            }
        }
    }

    Ok(time_taken)
}

fn assign_workers(mut workers: Vec<Option<u8>>, available_tasks: HashSet<u8>) -> Vec<Option<u8>> {
    let worked_tasks: HashSet<_> = workers.iter().filter_map(|task| *task).collect();
    let mut available_tasks: Vec<_> = available_tasks.difference(&worked_tasks).collect();

    let mut free_workers = vec![];
    for worker in 0..workers.len() {
        if let None = workers[worker] {
            free_workers.push(worker);
        }
    }

    for worker in free_workers {
        if let Some(&task) = available_tasks.pop() {
            workers[worker] = Some(task);
        }
    }

    workers
}

fn available_tasks(
    tasks: &HashSet<u8>,
    deps_map: &Dependencies,
    completed_tasks: &HashSet<u8>,
) -> HashSet<u8> {
    let available = |task: &&u8| {
        !completed_tasks.contains(&task)
            && deps_map.get(&task).map_or(true, |deps| {
                deps.iter().all(|dep| completed_tasks.contains(dep))
            })
    };

    tasks.iter().filter(available).map(|t| *t).collect()
}

fn parse_dependencies(input: &str) -> StdResult<Dependencies> {
    let mut deps_map = HashMap::new();
    for line in input.lines() {
        let (task, prereq) = parse_line(line)?;
        let deps = deps_map.entry(task).or_insert_with(HashSet::new);
        deps.insert(prereq);
    }

    Ok(deps_map)
}

fn parse_line(line: &str) -> StdResult<(u8, u8)> {
    let words: Vec<&str> = line.split_whitespace().collect();
    let prerequisite = words[1].bytes().next().ok_or("parse failure")?;
    let task = words[7].bytes().next().ok_or("parse failure")?;
    Ok((task, prerequisite))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.";

    #[test]
    fn parsing() {
        let deps = parse_dependencies(EXAMPLE_INPUT).unwrap();
        assert_eq!(deps.len(), 5);
        assert!(deps.get(&b'A').unwrap().contains(&b'C'));
        assert!(deps.get(&b'F').unwrap().contains(&b'C'));
        assert!(deps.get(&b'B').unwrap().contains(&b'A'));
        assert!(deps.get(&b'D').unwrap().contains(&b'A'));
        assert!(deps.get(&b'E').unwrap().contains(&b'B'));
        assert!(deps.get(&b'E').unwrap().contains(&b'D'));
        assert!(deps.get(&b'E').unwrap().contains(&b'F'));
    }

    #[test]
    fn part_one_example() {
        let tasks: HashSet<u8> = (b'A'..=b'F').collect();
        let result = part_one(EXAMPLE_INPUT, &tasks).unwrap();
        assert_eq!(result, "CABDFE".to_string());
    }

    #[test]
    fn part_two_example() {
        let tasks: HashSet<u8> = (b'A'..=b'F').collect();
        let result = part_two(EXAMPLE_INPUT, &tasks, 0, 2).unwrap();
        assert_eq!(result, 15);
    }
}
