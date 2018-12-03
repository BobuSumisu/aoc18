pub trait Day {
    fn number(&self) -> isize;
    fn name(&self) -> &str;
    fn part_one(&mut self, _input: &str) -> String { "(unimplemented)".to_owned() }
    fn part_two(&mut self, _input: &str) -> String { "(unimplemented)".to_owned() }
}
