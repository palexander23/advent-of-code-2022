use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn main() {
    print!("Loading Test Data...");
    let test_data_lines_vec = get_file_line_vec("data/day1_calorie_counting/test_data.txt").expect("Could not open test_data.txt");
    let real_data_lines_vec = get_file_line_vec("data/day1_calorie_counting/real_data.txt").expect("Could not open real_data.txt");
    println!("Loading complete");

    print!("Checking test data expected output...");
    let expected_test_data_output = (24000, 41000);
    let test_data_output = process_data(test_data_lines_vec);

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
    let real_data_output = process_data(real_data_lines_vec);
    println!("Complete");
    println!("Problem 1st Half: {}", real_data_output.0);
    println!("Problem 2nd Half: {}", real_data_output.1);
}

fn process_data(data_lines: Vec<String>) -> (u32, u32) {
    let mut calorie_sums: Vec<u32> = vec![];
    let mut current_sum = 0;

    for line in data_lines {
        if line.is_empty() {
            calorie_sums.push(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line.parse::<u32>().unwrap();
    }

    let top_calories = calorie_sums.iter().max().unwrap().clone();

    // Get the max three values
    let mut top_3_calories_sum = 0;
    for _ in 0..3 {
        let next_max = calorie_sums.iter().max().unwrap();
        top_3_calories_sum += next_max;

        if let Some(pos) = calorie_sums.iter().position(|x| *x == *next_max) {
            calorie_sums.remove(pos);
        }
    }

    return (top_calories, top_3_calories_sum);
}

fn get_file_line_vec(filepath: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|l| l.unwrap()).collect()) 
}
