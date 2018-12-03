use regex::Regex;
use day::Day;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(line: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let m = RE.captures(line).unwrap();
        Claim {
            id: m[1].parse::<usize>().unwrap(),
            left: m[2].parse::<usize>().unwrap(),
            top: m[3].parse::<usize>().unwrap(),
            width: m[4].parse::<usize>().unwrap(),
            height: m[5].parse::<usize>().unwrap(),
        }
    }

    fn right(&self) -> usize { self.left + self.width }
    fn bottom(&self) -> usize { self.top + self.height }

    fn overlaps(&self, other: &Claim) -> bool {
        self.left < other.right() && self.right() > other.left
            && self.top < other.bottom() && self.bottom() > other.top
    }
}

pub struct Day3;

impl Day for Day3 {
    fn number(&self) -> isize { 3 }
    fn name(&self) -> &str { "No Matter How You Slice It" }

    fn part_one(&mut self, input: &str) -> String {
        let mut fabric = vec![0; 1000*1000];
        input.lines()
            .map(|line| Claim::new(line))
            .for_each(|claim| {
                for y in 0..claim.height {
                    for x in 0..claim.width {
                        fabric[((claim.top + y) * 1000) + (claim.left + x)] += 1;
                    }
                }
            });

        fabric.iter()
            .filter(|&x| *x > 1)
            .count()
            .to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        let claims = input.lines()
            .map(|line| Claim::new(line))
            .collect::<Vec<_>>();

        for x in claims.iter() {
            let mut found = true;

            for y in claims.iter() {
                if x.id != y.id && x.overlaps(y) {
                    found = false;
                    break;
                }
            }

            if found {
                return x.id.to_string();
            }
        }

        "failed".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut day = Day3;
        let input = day.input();
        assert_eq!(day.part_one(&input), "110546");
    }

    #[test]
    fn test_part_two() {
        let mut day = Day3;
        let input = day.input();
        assert_eq!(day.part_two(&input), "819");
    }
}
