pub mod calories {
  use std::fs::read_to_string;

  pub fn find_highest_calorie() -> i32 {
    let input = read_to_string("./src/day1/input.txt").unwrap();
    let calories_per_elf = input.split("\n\n");

    calories_per_elf
      .map(|calories| {
        calories
          .split("\n")
          .map(|calorie| { calorie.parse::<i32>().unwrap() })
          .sum::<i32>()
      })
      .max()
        .unwrap()
  }

  pub fn find_top_three_calories() -> i32 {
    let input = read_to_string("./src/day1/input.txt").unwrap();
    let calories_per_elf = input.split("\n\n");

    let mut calorie_counts: Vec<i32> = calories_per_elf.map(|calories| {
      calories
        .split("\n")
        .map(|calorie| { calorie.parse::<i32>().unwrap() })
        .sum::<i32>()
    }).collect();
    
    calorie_counts.sort();
    calorie_counts.reverse();
    calorie_counts.iter().take(3).sum()
  }
}