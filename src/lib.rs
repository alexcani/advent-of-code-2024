pub mod util {
  pub mod grid;
  pub mod point;
  pub mod solution;
}

use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
  let file = File::open(filepath)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_vec(filepath: &str) -> io::Result<Vec<String>> {
  let lines = read_lines(filepath)?;
  Ok(lines.map_while(Result::ok).collect())
}

// Re-export modules for easy access
pub use util::grid::*;
pub use util::point::*;
pub use util::solution::*;
