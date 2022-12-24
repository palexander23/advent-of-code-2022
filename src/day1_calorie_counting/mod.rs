pub fn process_data(data_lines: Vec<String>) -> (String, String) {
    let mut calorie_sums: Vec<i32> = vec![];
    let mut current_sum = 0;

    for line in data_lines {
        if line.is_empty() {
            calorie_sums.push(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line.parse::<i32>().unwrap();
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

    return (top_calories.to_string(), top_3_calories_sum.to_string());
}
