use std::{fs::read_to_string, collections::HashSet};

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn priority_sum() -> i32 {
  let input = read_to_string("./src/day3/input.txt").unwrap();
  let rucksacks = input.split("\n");

  rucksacks.map(|rucksack| {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let left_unique = left.chars().collect::<HashSet<char>>();
    let right_unique = right.chars().collect::<HashSet<char>>();
    
    let common_item = left_unique.intersection(&right_unique).collect::<Vec<&char>>()[0];
    
    i32::try_from(PRIORITY.find(*common_item).unwrap()).unwrap() + 1
  }).sum()
}

pub fn find_badges_sum() -> i32 {
  let input = read_to_string("./src/day3/input.txt").unwrap();
  let rucksacks = input.split("\n").collect::<Vec<&str>>();

  rucksacks.chunks(3).map(|elf_group| {
    let unique_items = elf_group.iter()
      .map(|rucksack| { rucksack.chars().collect::<HashSet<char>>() })
      .reduce(|uniq, rucksack| {
        uniq.intersection(&rucksack)
          .map(|char| { *char } )
          .collect::<HashSet<char>>()
      }).unwrap();
  
    let badge = unique_items.iter().collect::<Vec<&char>>()[0];
    i32::try_from(PRIORITY.find(*badge).unwrap()).unwrap() + 1
  }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::priority_sum;
    use crate::day3::find_badges_sum;

  #[test]
  pub fn day3_part1() {
    assert_eq!(priority_sum(), 7766);
  }

  #[test]
  pub fn day3_part2() {
    assert_eq!(find_badges_sum(), 2415);
  }
}