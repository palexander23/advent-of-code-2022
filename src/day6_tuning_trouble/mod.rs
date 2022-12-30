use array_tool::vec::Uniq;

pub fn process_data(data_lines: Vec<String>) -> (String, String) {
    // Get the individual characters from the input line
    // All the input data is given on a single line
    let data_chars: Vec<char> = data_lines[0].chars().collect();

    // Iterate through the chars in chunks of 4
    let mut signal_start = 0;
    for i in 4..data_chars.len() {
        // Get the vector with the previous 4 chars
        let mut chunk_vec = data_chars[i - 4..i].to_vec();

        // Remove any duplicates and check if it is still of length 4
        chunk_vec = chunk_vec.unique();

        if chunk_vec.len() == 4 {
            signal_start = i;
            break;
        }
    }

    (signal_start.to_string(), "0".to_string())
}
