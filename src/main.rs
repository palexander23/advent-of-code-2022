use advent_of_code_2022::run_puzzle_script;

mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod day3_rucksack_reorganization;
mod day4_camp_cleanup;
mod day5_supply_stacks;

fn main() {
    run_puzzle_script(
        day1_calorie_counting::process_data,
        ("24000".to_string(), "41000".to_string()),
        "day1_calorie_counting".to_string(),
    );

    run_puzzle_script(
        day2_rock_paper_scissors::process_data,
        ("15".to_string(), "12".to_string()),
        "day2_rock_paper_scissors".to_string(),
    );

    run_puzzle_script(
        day3_rucksack_reorganization::process_data,
        ("157".to_string(), "70".to_string()),
        "day3_rucksack_reorganization".to_string(),
    );

    run_puzzle_script(
        day4_camp_cleanup::process_data,
        ("2".to_string(), "4".to_string()),
        "day4_camp_cleanup".to_string(),
    );

    run_puzzle_script(
        day5_supply_stacks::process_data,
        ("CMZ".to_string(), "MCD".to_string()),
        "day5_supply_stacks".to_string(),
    );
}
