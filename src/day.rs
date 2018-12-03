use std::fs::File;
use std::io::prelude::*;

pub trait Day {
    fn number(&self) -> isize;
    fn name(&self) -> &str;
    fn part_one(&mut self, _input: &str) -> String { "unimplemented".to_owned() }
    fn part_two(&mut self, _input: &str) -> String { "unimplemented".to_owned() }

    fn input(&self) -> String {
        let mut input = String::new();
        File::open(format!("input/day{}.txt", self.number())).unwrap().read_to_string(&mut input).unwrap();
        input
    }
}
