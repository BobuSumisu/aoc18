#[macro_use] extern crate lazy_static;
extern crate regex;

pub mod day;
pub mod day1;
pub mod day2;
pub mod day3;

use std::fs::File;
use std::io::prelude::*;
use day::Day;

fn run(day: &mut Day) {
    let mut input = String::new();
    File::open(format!("input/day{}.txt", day.number())).unwrap().read_to_string(&mut input).unwrap();
    println!("\n--- Day {}: {} ---\n", day.number(), day.name());
    println!("  Part One: {:?}", day.part_one(&input));
    println!("  Part Two: {:?}", day.part_two(&input));
}

fn main() {
    let mut days: [Box<Day>; 3] = [
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
    ];

    days.iter_mut()
        .for_each(|day| run(day.as_mut()));
}
