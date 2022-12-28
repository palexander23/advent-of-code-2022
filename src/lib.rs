use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

pub fn run_puzzle_script(
    puzzle_script: fn(Vec<String>) -> (String, String),
    expected_outputs: (String, String),
    folder_name: String,
) {
    println!("\n\n*----------------------------------------*");
    println!("* {}", folder_name);
    println!("*----------------------------------------*");
    print!("Loading Test Data...");
    let test_data_lines_vec = get_file_line_vec(&format!("src/{}/test_data.txt", folder_name))
        .expect("Could not open test_data.txt");
    let real_data_lines_vec = get_file_line_vec(&format!("src/{}/real_data.txt", folder_name))
        .expect("Could not open real_data.txt");
    println!("Loading complete");

    print!("Checking test data expected output...");
    let test_data_output = puzzle_script(test_data_lines_vec);

    if expected_outputs != test_data_output {
        println!("Failed!");
        println!("Problem 1st Half: ");
        println!("Expected: {}", expected_outputs.0);
        println!("Actual:   {}", test_data_output.0);
        println!("Problem 2nd Half:");
        println!("Expected: {}", expected_outputs.1);
        println!("Actual:   {}", test_data_output.1);
        exit(1);
    } else {
        println!("Success!");
    }

    print!("\nComputing Real Answer...");
    let real_data_output = puzzle_script(real_data_lines_vec);
    println!("Complete");
    println!("Problem 1st Half: {}", real_data_output.0);
    println!("Problem 2nd Half: {}", real_data_output.1);
}

fn get_file_line_vec(filepath: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|l| l.unwrap()).collect())
}
