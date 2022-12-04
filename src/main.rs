mod day1;

use day1::calories::find_highest_calorie;
use day1::calories::find_top_three_calories;

fn main() {
    println!("{}", find_highest_calorie());
    println!("{}", find_top_three_calories());
}
