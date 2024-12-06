pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

use advent_of_code_2024::Context;

pub fn get_day_solver(day: u8) -> fn(&mut Context) -> () {
  match day {
      1 => day01::solve,
      2 => day02::solve,
      3 => day03::solve,
      4 => day04::solve,
      5 => day05::solve,
      6 => day06::solve,
      _ => unimplemented!(),
  }
}
