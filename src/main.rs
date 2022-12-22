use advent_of_code_2022::run_puzzle_script;

mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod day3_rucksack_reorganization;
mod day4_camp_cleanup;

fn main() {
    run_puzzle_script(
        day1_calorie_counting::process_data,
        (24000, 41000),
        "day1_calorie_counting".to_string(),
    );

    run_puzzle_script(
        day2_rock_paper_scissors::process_data,
        (15, 12),
        "day2_rock_paper_scissors".to_string(),
    );

    run_puzzle_script(
        day3_rucksack_reorganization::process_data,
        (157, 70),
        "day3_rucksack_reorganization".to_string(),
    );

    run_puzzle_script(
        day4_camp_cleanup::process_data,
        (0, 0),
        "day4_camp_cleanup".to_string(),
    );
}
