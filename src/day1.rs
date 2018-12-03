use std::collections::HashSet;
use day::Day;

pub struct Day1;

impl Day for Day1 {
    fn number(&self) -> isize { 1 }
    fn name(&self) -> &str { "Chronal Calibration" }

    fn part_one(&mut self, input: &str) -> String {
        input.lines()
            .fold(0, |acc, line| acc + line.parse::<isize>().unwrap())
            .to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        let numbers: Vec<_> = input.lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();

        let mut seen = HashSet::new();
        let mut acc = 0;

        loop {
            for n in numbers.iter() {
                acc += n;

                if seen.contains(&acc) {
                    return acc.to_string();
                }

                seen.insert(acc);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut day1 = Day1;
        let input = day1.input();
        assert_eq!(day1.part_one(&input), "493");
    }

    #[test]
    fn test_part_two() {
        let mut day1 = Day1;
        let input = day1.input();
        assert_eq!(day1.part_two(&input), "413");
    }
}
