use day::Day;

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

pub struct Day2;

impl Day for Day2 {
    fn number(&self) -> isize { 2 }
    fn name(&self) -> &str { "Inventory Management System" }

    fn part_one(&mut self, input: &str) -> String {
        let v = input.lines()
            .map(|line| count_letters(&line))
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        (v.0 * v.1).to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();

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
}
