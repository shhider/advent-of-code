use std::{fs};
use std::io::{BufRead, BufReader};

fn main() {
  let eg = vec![
    "vJrwpWtwJgWrhcsFMMfFFhFp",
    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    "PmmdzqPrVvPwwTWBwg",
    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    "ttgJtRGJQctTZtZT",
    "CrZsJsPPZsGzwwsLwLmpwMDw"
  ];
  let eggg = eg.iter().map(|&x| x).map(|x| String::from(x)).collect();

  part1_solve(&eggg);
  part1_solve(&get_input());
}

// #region Part2
fn part2_solve(list: &Vec<String>) -> u32 {
  // let group: [String: 3]
}

fn part2_deal_rucksack_group() -> u32 {
  //
}
// #endregion

// #region Part1
fn part1_solve(list: &Vec<String>) -> u32 {
  let sum = list.iter()
    .map(|x| part1_deal_rucksack(&x))
    .sum();

  println!("sum: {sum}");
  sum
}

fn part1_deal_rucksack(rucksack: &String) -> u32 {
  let mut chars: Vec<char> = rucksack.chars().collect();
  let len = chars.len();
  let half = len / 2;
  let right: Vec<char> = chars.drain(half..).collect();

  // let left_str = String::from_iter(&chars);
  // let right_str = String::from_iter(&right);
  // println!("left: {left_str}");
  // println!("right: {right_str}");

  let mut result = vec![];

  right.iter().for_each(|item| {
    if !result.contains(&item) && chars.contains(&item) {
      result.push(item);
    }
  });

  result.iter().map(|i| letter_to_num(&i)).sum()
}
// #endregion

fn letter_to_num(letter: &char) -> u32 {
  // println!("letter {letter}");
  let num = letter.clone() as u32;
  if num > 96 { num - 96 } else { num - (65 - 27) }
}

const INPUT_FILE: &str = "./input.txt";

fn get_input() -> Vec<String> {
  let mut result = vec![];

  let ipt_file = fs::File::open(INPUT_FILE).expect("invalid input file");
  let ipt_reader = BufReader::new(ipt_file);
  for line in ipt_reader.lines() {
    if let Ok(ll) = line {
      result.push(String::from(ll.trim()));
    }
  }
  result
}
