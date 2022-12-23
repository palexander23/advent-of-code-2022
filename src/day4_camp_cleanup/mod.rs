use regex::Regex;

pub fn process_data(data_lines: Vec<String>) -> (i32, i32) {
    // Convert the incoming strings to integer ranges
    let ranges_vec: Vec<((i32, i32), (i32, i32))> = data_lines
        .iter()
        .map(|line| get_int_ranges_from_input_line(line))
        .collect();

    // Sum up the number of pairs that overlap
    let overlaps_sum: i32 = ranges_vec
        .iter()
        .map(|r| {
            let ((l1, h1), (l2, h2)) = r;
            check_overlap(l1, h1, l2, h2)
        })
        .map(|b| b as i32)
        .sum();

    (overlaps_sum, 0)
}

fn get_int_ranges_from_input_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let range_regex = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();

    let caps = range_regex.captures(&line).unwrap();
    (
        (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        ),
        (
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        ),
    )
}

fn check_overlap(l1: &i32, h1: &i32, l2: &i32, h2: &i32) -> bool {
    // Check if l2-h2 is inside l1-h1
    if l2 >= l1 && h2 <= h1 {
        return true;
    }

    // Check if l1-h1 is inside l2-h2
    if l1 >= l2 && h1 <= h2 {
        return true;
    }

    return false;
}
