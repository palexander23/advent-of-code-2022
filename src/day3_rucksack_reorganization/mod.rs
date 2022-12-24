use std::char;

pub fn process_data(data_lines: Vec<String>) -> (String, String) {
    // Declare a vector to hold the shared item in each line
    let mut shared_char_vec = vec!['@'; data_lines.len()];

    // Split the lines down the middle
    let split_data_lines: Vec<(&str, &str)> =
        data_lines.iter().map(|l| l.split_at(l.len() / 2)).collect();

    // Save the line the characters have in common
    for (i, (line1, line2)) in split_data_lines.iter().enumerate() {
        for char in line1.chars() {
            if line2.contains(char) {
                shared_char_vec[i] = char;
            }
        }
    }

    // Convert the characters into their numerical values counting up from a = 1
    let mut part1_score_sum: i32 = 0;
    for c in shared_char_vec {
        part1_score_sum += get_char_priority(c);
    }

    // Part 2
    // Split the elves into their groups of three
    let part2_score_sum: i32 = data_lines
        .chunks(3)
        .map(|g| {
            for c in g[0].chars() {
                if g[1].contains(c) && g[2].contains(c) {
                    return get_char_priority(c);
                }
            }
            panic!("Found a group with no common character!")
        })
        .sum();
    (part1_score_sum.to_string(), part2_score_sum.to_string())
}

fn get_char_priority(c: char) -> i32 {
    if c.is_uppercase() {
        (c as u8 - 38) as i32
    } else {
        (c as u8 - 96) as i32
    }
}
