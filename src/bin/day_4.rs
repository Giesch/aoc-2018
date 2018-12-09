#![feature(slice_patterns)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type StdResult<T> = Result<T, Box<std::error::Error>>;

fn main() -> StdResult<()> {
    let input = include_str!("../input/day_4_input.txt");

    let solution_one = part_one(input)?;
    println!("Part One: {}", solution_one);

    Ok(())
}

fn parse_log_lines(input: &str) -> StdResult<Vec<LogLine>> {
    input.lines().map(LogLine::parse).collect()
}

#[derive(Clone, Debug)]
enum FallWake {
    Fall(TimeStamp),
    Wake(TimeStamp),
}

#[derive(Debug, Clone)]
struct NightData {
    guard_id: i32,
    events: Vec<FallWake>,
}

// map of guard id to minutes asleep
fn sleep_map_entry(data: &NightData) -> (i32, HashSet<i32>) {
    let mut minutes = HashSet::new();
    let mut fell_asleep = 0;
    for event in &data.events {
        match event {
            FallWake::Fall(ts) => {
                fell_asleep = ts.minute;
            }
            FallWake::Wake(ts) => {
                for m in fell_asleep..ts.minute {
                    minutes.insert(m);
                }
            }
        }
    }

    (data.guard_id, minutes)
}

// guard id to map of minute of hour to minutes slept
fn sleep_map(data: Vec<NightData>) -> HashMap<i32, HashMap<i32, i32>> {
    let mut result = HashMap::new();
    for datum in &data {
        let (guard_id, minutes) = sleep_map_entry(datum);
        let entry = result.entry(guard_id).or_insert_with(|| HashMap::new());
        for m in &minutes {
            let minutes_slept: &mut i32 = (*entry).entry(*m).or_insert(0 as i32);
            *minutes_slept += 1;
        }
    }

    result
}

fn to_night_data(input: &str) -> StdResult<Vec<NightData>> {
    let mut lines = parse_log_lines(input)?;
    lines.sort_by(|x, y| x.timestamp().cmp(y.timestamp()));

    let mut result = vec![];
    let mut current_data: Option<NightData> = None;
    for line in &lines {
        match line {
            LogLine::Guard(id, _ts) => {
                if let Some(data) = current_data {
                    result.push(data.clone());
                }

                current_data = Some(NightData {
                    guard_id: *id,
                    events: vec![],
                });
            }
            LogLine::Fall(ts) => {
                if let Some(ref mut data) = current_data {
                    data.events.push(FallWake::Fall(ts.clone()));
                }
            }
            LogLine::Wake(ts) => {
                if let Some(ref mut data) = current_data {
                    data.events.push(FallWake::Wake(ts.clone()));
                }
            }
        }
    }

    Ok(result)
}

fn part_one(input: &str) -> StdResult<i32> {
    let sleep_map = sleep_map(to_night_data(input)?);

    let guard_id = sleepiest_guard(&sleep_map)?;
    let our_guy = sleep_map
        .get(&guard_id)
        .ok_or("sleepiest guard is broken")?;

    let (minute, _) = our_guy
        .iter()
        .max_by(|x, y| {
            let (_left_k, left_v) = x;
            let (_right_k, right_v) = y;
            left_v.cmp(right_v)
        })
        .ok_or("empty guard data")?;

    Ok(minute * guard_id)
}

fn sleepiest_guard(sleep_map: &HashMap<i32, HashMap<i32, i32>>) -> StdResult<i32> {
    let (k, _v) = sleep_map
        .iter()
        .map(|(k, v)| (k, v.values().sum::<i32>()))
        .max_by(|x, y| {
            let (_, lv) = x;
            let (_, rv) = y;
            lv.cmp(rv)
        })
        .ok_or("empty data")?;

    Ok(*k)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TimeStamp {
    year: u32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

impl TimeStamp {
    fn parse(day_chunk: &str, time_chunk: &str) -> StdResult<TimeStamp> {
        let mut timestamp = TimeStamp::default();

        let mut day_chunk: String = day_chunk.to_string();
        day_chunk.remove(0);
        let day_v: Vec<&str> = day_chunk.split("-").collect();
        match day_v.as_slice() {
            [y, m, d] => {
                timestamp.year = y.parse()?;
                timestamp.month = m.parse()?;
                timestamp.day = d.parse()?;
            }
            _ => Err("parse date failed")?,
        }

        let mut time_chunk = time_chunk.to_string();
        time_chunk.pop();
        let time_v: Vec<&str> = time_chunk.split(":").collect();
        match time_v.as_slice() {
            [h, m] => {
                timestamp.hour = h.parse()?;
                timestamp.minute = m.parse()?;
            }
            _ => Err("parse time failed")?,
        }

        Ok(timestamp)
    }
}

enum LogLine {
    Guard(i32, TimeStamp),
    Fall(TimeStamp),
    Wake(TimeStamp),
}

impl LogLine {
    fn parse(line: &str) -> StdResult<LogLine> {
        let v: Vec<&str> = line.split_whitespace().collect();
        match v.as_slice() {
            [d, t, w, i, _rest..] => {
                let ts = TimeStamp::parse(d, t)?;
                if w.starts_with('G') {
                    let mut i = i.to_string();
                    i.remove(0);
                    let id: i32 = i.parse()?;
                    Ok(LogLine::Guard(id, ts))
                } else if w.starts_with('f') {
                    Ok(LogLine::Fall(ts))
                } else if w.starts_with('w') {
                    Ok(LogLine::Wake(ts))
                } else {
                    Err("parse log line failed")?
                }
            }
            _ => Err("parse log line failed")?,
        }
    }

    fn timestamp(&self) -> &TimeStamp {
        match self {
            LogLine::Guard(_, ts) => ts,
            LogLine::Fall(ts) => ts,
            LogLine::Wake(ts) => ts,
        }
    }
}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeStamp {
    fn cmp(&self, other: &Self) -> Ordering {
        let years = self.year.cmp(&other.year);
        if years != Ordering::Equal {
            return years;
        }
        let months = self.month.cmp(&other.month);
        if months != Ordering::Equal {
            return months;
        }
        let days = self.day.cmp(&other.day);
        if days != Ordering::Equal {
            return days;
        }
        let hours = self.hour.cmp(&other.hour);
        if hours != Ordering::Equal {
            return hours;
        }
        self.minute.cmp(&other.minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_timestamp() {
        let input = "[1518-11-01 00:00]";
        let v: Vec<&str> = input.split_whitespace().collect();
        let timestamp = match v.as_slice() {
            [day_chunk, time_chunk] => TimeStamp::parse(day_chunk, time_chunk).unwrap(),
            _ => panic!(),
        };

        assert_eq!(timestamp.year, 1518);
        assert_eq!(timestamp.month, 11);
        assert_eq!(timestamp.day, 1);
        assert_eq!(timestamp.hour, 0);
        assert_eq!(timestamp.minute, 0);
    }

    #[test]
    fn parse_guard() {
        let input = "[1518-11-01 23:58] Guard #99 begins shift";
        let (id, ts) = match LogLine::parse(input).unwrap() {
            LogLine::Guard(id, ts) => (id, ts),
            _ => panic!(),
        };

        assert_eq!(id, 99);
        assert_eq!(ts.year, 1518);
        assert_eq!(ts.month, 11);
        assert_eq!(ts.day, 1);
        assert_eq!(ts.hour, 23);
        assert_eq!(ts.minute, 58);
    }

    #[test]
    fn part_1_example() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up";
        let result = part_one(input).unwrap();
        assert_eq!(240, result);
    }
}