pub mod calories {
  use std::fs::read_to_string;

  pub fn find_highest_calorie() -> i32 {
    let input = read_to_string("./src/day1/input.txt").unwrap();
    let calories_per_elf = input.split("\n\n");
    let mut highest_calorie = 0;

    for calories in calories_per_elf {
      let total_calories = calories
        .split("\n")
        .map(|calorie| { calorie.parse::<i32>().unwrap() })
        .sum();
      
      if total_calories > highest_calorie {
        highest_calorie = total_calories;
      }
    }

    highest_calorie
  }

  pub fn find_top_three_calories() -> i32 {
    let input = read_to_string("./src/day1/input.txt").unwrap();
    let calories_per_elf = input.split("\n\n");
    let mut calorie_counts = Vec::new();

    for calories in calories_per_elf {
      let total_calories = calories
        .split("\n")
        .map(|calorie| { calorie.parse::<i32>().unwrap() })
        .sum();

      calorie_counts.push(total_calories);
    }
    
    calorie_counts.sort();
    calorie_counts.reverse();
    calorie_counts.iter().take(3).sum()
  }
}