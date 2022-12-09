use std::fs;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "./input.txt";

fn main() {
  let ipt_file = fs::File::open(INPUT_FILE).expect("invalid input file");
  let ipt_reader = BufReader::new(ipt_file);

  let mut result: u32 = 0;
  let mut count: u32 = 0;
  for line in ipt_reader.lines() {
    if let Ok(ll) = line {
      if ll == "" {
        result = result.max(count);
        count = 0;
      } else {
        let item: u32 = ll.trim().parse::<u32>().expect("should be a number");
        count += item;
      }
    }
  }

  println!("max Calorie: {}", result);
}
