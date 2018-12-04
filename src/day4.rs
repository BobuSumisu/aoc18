use std::collections::HashMap;
use regex::Regex;
use day::Day;
use chrono::prelude::*;

#[derive(Debug)]
enum RecordData {
    WakesUp,
    FallsAsleep,
    NewShift(usize),
}

#[derive(Debug)]
struct Record {
    time: DateTime<Utc>,
    data: RecordData,
}

#[derive(Debug)]
struct Sleep {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Sleep {
    fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Sleep {
        Sleep {
            from: from,
            to: to,
        }
    }
}

#[derive(Debug)]
struct Guard {
    id: usize,
    sleep: Vec<Sleep>,
}

impl Guard {
    fn new(id: usize) -> Guard {
        Guard {
            id: id,
            sleep: Vec::new(),
        }
    }

    fn total_sleep(&self) -> usize {
        self.sleep.iter()
            .map(|s| s.to - s.from)
            .map(|d| d.num_minutes() as usize)
            .sum()
    }

    fn minute_most_asleep(&self) -> (usize, usize) {
        let mut minutes: [usize; 60] = [0; 60];

        for sleep in self.sleep.iter() {
            for min in sleep.from.minute()..sleep.to.minute() {
                minutes[min as usize] += 1;
            }
        }

        let mut i = 0;
        let mut max = 0;

        for (j, &n) in minutes.iter().enumerate() {
            if n > max {
                max = n;
                i = j;
            }
        }

        (i, max)
    }
}

impl Record {
    fn new(line: &str) -> Record {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[(.*)\] (.*)$").unwrap();
            static ref RE2: Regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
        }

        let m = RE.captures(line).unwrap();

        let data = match &m[2] {
            "wakes up" => RecordData::WakesUp,
            "falls asleep" => RecordData::FallsAsleep,
            s => {
                let m = RE2.captures(s).unwrap();
                RecordData::NewShift(m[1].parse::<usize>().unwrap())
            }
        };

        Record {
            time: Utc.datetime_from_str(&m[1], "%Y-%m-%d %H:%M").unwrap(),
            data: data,
        }
    }
}

fn parse_records(records: Vec<Record>) -> HashMap<usize, Guard> {
    let mut guards = HashMap::new();
    let mut curr_id = 0;
    let mut sleep_start = Utc::now();

    for record in records {
        match record.data {
            RecordData::NewShift(id) => {
                if !guards.contains_key(&id) {
                    let guard = Guard::new(id);
                    guards.insert(id, guard);
                }
                curr_id = id;
            },
            RecordData::FallsAsleep => {
                sleep_start = record.time;
            },
            RecordData::WakesUp => {
                guards.get_mut(&curr_id).unwrap()
                    .sleep.push(Sleep::new(sleep_start, record.time));
            },
        }
    }

    guards
}

pub struct Day4;

impl Day for Day4 {
    fn number(&self) -> isize { 4 }
    fn name(&self) -> &str { "Repose Record" }

    fn part_one(&mut self, input: &str) -> String {
        let mut records = input.lines()
            .map(|line| Record::new(line))
            .collect::<Vec<_>>();
        records.sort_by_key(|r| r.time);

        let guards = parse_records(records);
        let mut guards = guards.values().collect::<Vec<_>>();
        guards.sort_by_key(|g| g.total_sleep());
        guards.reverse();

        (guards[0].id * guards[0].minute_most_asleep().0).to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        let mut records = input.lines()
            .map(|line| Record::new(line))
            .collect::<Vec<_>>();
        records.sort_by_key(|r| r.time);

        let guards = parse_records(records);
        let mut guards = guards.values().collect::<Vec<_>>();
        guards.sort_by_key(|g| g.minute_most_asleep().1);
        guards.reverse();

        (guards[0].id * guards[0].minute_most_asleep().0).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut day = Day4;
        let input = day.input();
        assert_eq!(day.part_one(&input), "106710");
    }

    #[test]
    fn test_part_two() {
        let mut day = Day4;
        let input = day.input();
        assert_eq!(day.part_two(&input), "10491");
    }
}
