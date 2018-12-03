use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_letters(line: &str) -> (isize, isize) {
    let mut two = 0;
    let mut three = 0;

    for x in line.chars() {
        let c = line.matches(x).count();
        if c == 2 {
            two = 1;
        } else if c == 3 {
            three = 1;
        }
    }

    (two, three)
}

fn edit_distance(a: &str, b: &str) -> isize {
    let mut dist = 0;

    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            dist += 1;
        }
    }

    dist
}

pub fn part_one() -> isize {
    let v =
        BufReader::new(File::open("input/day2.txt").unwrap())
        .lines()
        .map(|line| count_letters(&line.unwrap()))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    v.0 * v.1
}

pub fn part_two() -> String {
    let lines: Vec<_> =
        BufReader::new(File::open("input/day2.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    for a in lines.iter() {
        for b in lines.iter() {
            if edit_distance(&a, &b) == 1 {
                return a.chars()
                    .zip(b.chars())
                    .filter(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect::<String>();
            }
        }
    }

    "(none)".to_owned()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 5952);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), "krdmtuqjgwfoevnaboxglzjph");
    }
}
