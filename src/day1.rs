use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_one() -> isize {
    BufReader::new(File::open("input/day1.txt").unwrap())
        .lines()
        .fold(0, |acc, line| acc + line.unwrap().parse::<isize>().unwrap())
}

pub fn part_two() -> isize {
    let numbers: Vec<_> =
        BufReader::new(File::open("input/day1.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect();
    let mut seen = HashSet::new();
    let mut acc = 0;

    loop {
        for n in numbers.iter() {
            acc += n;

            if seen.contains(&acc) {
                return acc
            }
            seen.insert(acc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 493);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 413);
    }
}
