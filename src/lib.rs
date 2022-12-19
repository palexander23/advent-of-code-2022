use std::{fs::File, io::{BufReader, BufRead}, process::exit};

mod day1_calorie_counting;

pub fn run_puzzle_script() {
    print!("Loading Test Data...");
    let test_data_lines_vec = get_file_line_vec("src/day1_calorie_counting/test_data.txt").expect("Could not open test_data.txt");
    let real_data_lines_vec = get_file_line_vec("src/day1_calorie_counting/real_data.txt").expect("Could not open real_data.txt");
    println!("Loading complete");

    print!("Checking test data expected output...");
    let expected_test_data_output = (24000, 41000);
    let test_data_output = day1_calorie_counting::process_data(test_data_lines_vec);

    if expected_test_data_output != test_data_output {
        println!("Failed!");
        println!("Problem 1st Half: ");
        println!("Expected: {}", expected_test_data_output.0);
        println!("Actual:   {}", test_data_output.0);
        println!("Problem 2nd Half:");
        println!("Expected: {}", expected_test_data_output.1);
        println!("Actual:   {}", test_data_output.1);
        exit(1);
    } else {
        println!("Success!");
    }

    print!("\nComputing Real Answer...");
    let real_data_output = day1_calorie_counting::process_data(real_data_lines_vec);
    println!("Complete");
    println!("Problem 1st Half: {}", real_data_output.0);
    println!("Problem 2nd Half: {}", real_data_output.1);
}

fn get_file_line_vec(filepath: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader =
     BufReader::new(file);

    Ok(reader.lines().map(|l| l.unwrap()).collect()) 
}

