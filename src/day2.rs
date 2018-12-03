use day::Day;

fn count_letters(line: &str) -> (isize, isize) {
    let mut two = 0;
    let mut three = 0;

    for x in line.chars() {
        match line.matches(x).count() {
            2 => two = 1,
            3 => three = 1,
            _ => (),
        }

        if two == 1 && three == 1 {
            break;
        }
    }

    (two, three)
}

fn edit_distance(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .filter(|(x, y)| x != y)
        .count()
}

pub struct Day2;

impl Day for Day2 {
    fn number(&self) -> isize { 2 }
    fn name(&self) -> &str { "Inventory Management System" }

    fn part_one(&mut self, input: &str) -> String {
        let count = input.lines()
            .map(|line| count_letters(&line))
            .fold((0, 0), |acc, c| (acc.0 + c.0, acc.1 + c.1));
        (count.0 * count.1).to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        for a in input.lines() {
            for b in input.lines() {
                if edit_distance(&a, &b) == 1 {
                    return a.chars()
                        .zip(b.chars())
                        .filter(|(x, y)| x == y)
                        .map(|(x, _)| x)
                        .collect::<String>();
                }
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
        let mut day2 = Day2;
        let input = day2.input();
        assert_eq!(day2.part_one(&input), "5952");
    }

    #[test]
    fn test_part_two() {
        let mut day2 = Day2;
        let input = day2.input();
        assert_eq!(day2.part_two(&input), "krdmtuqjgwfoevnaboxglzjph");
    }
}
