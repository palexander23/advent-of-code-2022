
use advent_of_code_2022::run_puzzle_script;

mod day1_calorie_counting;



fn main() {
    run_puzzle_script(day1_calorie_counting::process_data, (24000, 41000),"day1_calorie_counting".to_string() );

}

