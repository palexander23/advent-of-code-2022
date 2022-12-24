use lazy_static::lazy_static;
use regex::Regex;

pub fn process_data(data_lines: Vec<String>) -> (i32, i32) {
    // Convert the incoming strings to integer ranges
    let ranges_vec: Vec<((i32, i32), (i32, i32))> = data_lines
        .iter()
        .map(|line| get_int_ranges_from_input_line(line))
        .collect();

    // Sum up the number of pairs that overlap
    let contains_sum: i32 = ranges_vec
        .iter()
        .map(|r| {
            let ((l1, h1), (l2, h2)) = r;
            check_contains(l1, h1, l2, h2)
        })
        .map(|b| b as i32)
        .sum();

    // Part Two
    let overlaps_sum: i32 = ranges_vec
        .iter()
        .map(|r| {
            let ((l1, h1), (l2, h2)) = r;
            check_overlap(l1, h1, l2, h2)
        })
        .map(|b| b as i32)
        .sum();

    (contains_sum, overlaps_sum)
}

fn get_int_ranges_from_input_line(line: &str) -> ((i32, i32), (i32, i32)) {
    lazy_static! {
        static ref RANGE_REGEX: Regex = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    }

    let caps = RANGE_REGEX.captures(&line).unwrap();
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

fn check_contains(l1: &i32, h1: &i32, l2: &i32, h2: &i32) -> bool {
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

fn check_overlap(l1: &i32, h1: &i32, l2: &i32, h2: &i32) -> bool {
    // Check inner overlap
    if l2 <= h1 && h2 >= l1 {
        return true;
    }

    return false;
}
