use std::fs::read_to_string;
use std::collections::HashMap;

pub fn my_strategy_total() -> i32 {
  struct HandAttributes {
    trumps: String,
    score: i32,
  }

  let mut game_rules: HashMap<String, HandAttributes> = HashMap::new();
  game_rules.insert(String::from("A"), HandAttributes { trumps: String::from("Z"), score: 1 });
  game_rules.insert(String::from("B"), HandAttributes { trumps: String::from("X"), score: 2 });
  game_rules.insert(String::from("C"), HandAttributes { trumps: String::from("Y"), score: 3 });
  game_rules.insert(String::from("X"), HandAttributes { trumps: String::from("C"), score: 1 });
  game_rules.insert(String::from("Y"), HandAttributes { trumps: String::from("A"), score: 2 });
  game_rules.insert(String::from("Z"), HandAttributes { trumps: String::from("B"), score: 3 });

  let input = read_to_string("./src/day2/input.txt").unwrap();
  let turns = input.split("\n");

  turns.map(|turn| {
    let individual_hands: Vec<&str> = turn.split(" ").collect();
    let elf_hand = game_rules.get(individual_hands[0]).unwrap();
    let my_hand = game_rules.get(individual_hands[1]).unwrap();

    if my_hand.trumps == individual_hands[0] {
      return 6 + my_hand.score;
    } else if elf_hand.trumps == individual_hands[1] {
      return my_hand.score;
    } else {
      return 3 + my_hand.score;
    }
  }).sum()
}

pub fn elf_strategy_total() -> i32 {
  struct StrategyGuide {
    score: i32,
    to_win_score: i32,
    to_lose_score: i32,
  }

  let mut game_guide: HashMap<String, StrategyGuide> = HashMap::new();
  game_guide.insert(String::from("A"), StrategyGuide { score: 1, to_win_score: 2, to_lose_score: 3 });
  game_guide.insert(String::from("B"), StrategyGuide { score: 2, to_win_score: 3, to_lose_score: 1 });
  game_guide.insert(String::from("C"), StrategyGuide { score: 3, to_win_score: 1, to_lose_score: 2 });

  let input = read_to_string("./src/day2/input.txt").unwrap();
  let turns = input.split("\n");
  
  turns.map(|turn| {
    let individual_hands: Vec<&str> = turn.split(" ").collect();
    match individual_hands[1] {
      "X" => game_guide.get(individual_hands[0]).unwrap().to_lose_score,
      "Y" => game_guide.get(individual_hands[0]).unwrap().score + 3,
      "Z" => game_guide.get(individual_hands[0]).unwrap().to_win_score + 6,
        _ => 0,
    }
  }).sum()
}

#[cfg(test)]
pub mod tests {
  use crate::day2::my_strategy_total;
  use crate::day2::elf_strategy_total;
  #[test]
  fn test_day2_part1() {
    assert_eq!(elf_strategy_total(), 13193);
  }

  #[test]
  fn test_day2_part2() {
    assert_eq!(my_strategy_total(), 12586);
  }
}
