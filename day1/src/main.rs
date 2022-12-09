use std::fs;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "./input.txt";

fn main() {
  let ipt_file = fs::File::open(INPUT_FILE).expect("invalid input file");
  let ipt_reader = BufReader::new(ipt_file);

  let mut top3: Vec<u32> = vec![0, 0, 0];
  let mut the_max: u32 = 0;
  let mut count: u32 = 0;
  for line in ipt_reader.lines() {
    if let Ok(ll) = line {
      if ll.is_empty() {
        let num3 = top3.pop().expect("unexpected").max(count);
        top3.push(num3);
        top3.sort();
        top3.reverse();

        the_max = the_max.max(count);

        count = 0;
      } else {
        let item: u32 = ll.trim().parse::<u32>().expect("should be a number");
        count += item;
      }
    }
  }

  let top3_total: u32 = top3.iter().sum();

  println!("max Calorie: {}", the_max);
  println!("total calorie of top3: {}", top3_total);
}
