use std::fs;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "./input.txt";

fn main() {
  // try temp input
  let mut temp = Calculator::new();
  temp = temp.add_round("A", "Y");
  temp = temp.add_round("B", "X");
  temp = temp.add_round("C", "Z");
  println!("temp score: {}", temp.get_score());

  // the test input
  let ipt_file = fs::File::open(INPUT_FILE).expect("invalid input file");
  let ipt_reader = BufReader::new(ipt_file);

  let mut calc = Calculator::new();
  for line in ipt_reader.lines() {
    if let Ok(ll) = line {
      let shapes: Vec<&str> = ll.trim().split(" ").collect();
      let oppo = shapes.get(0).expect("first shape should be existed");
      let me = shapes.get(1).expect("second shape should be existed");
      calc = calc.add_round(oppo, me);
    }
  }

  println!("my score: {}", calc.get_score());
}

struct Calculator {
  value: u32,
}

impl Calculator {
  fn new() -> Calculator {
    Self { value: 0 }
  }

  fn add_round(mut self, oppo: &str, me: &str) -> Self {
    self.value += get_score_of_shape(me);
    self.value += get_score_of_outcome(oppo, me);
    self
  }

  fn get_score(self) -> u32 {
    self.value
  }
}


fn get_score_of_shape(shape: &str) -> u32 {
  if shape == "X" { return 1; }
  else if shape == "Y" { return 2; }
  else if shape == "Z" { return 3; }
  0
}

fn get_score_of_outcome(oppo: &str, me: &str) -> u32 {
  if me == "X" { // rock
    if oppo == "C" { return 6; }
    else if oppo == "A" { return 3; }
  } else if me == "Y" { // paper
    if oppo == "A" { return 6; }
    else if oppo == "B" { return 3; }
  } else if me == "Z" { // scissor
    if oppo == "B" { return 6; }
    else if oppo == "C" { return 3; }
  }

  0
}
