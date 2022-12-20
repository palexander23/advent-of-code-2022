const DRAW: i32 = 3;
const WIN: i32 = 6;
const LOOSE: i32 = 0;

pub fn process_data(data_lines: Vec<String>) -> (i32, i32) {
    let mut part_1_total_score = 0i32;
    let mut part_2_total_score = 0i32;

    for line in &data_lines {
        let their_hand = match line
            .chars()
            .nth(0)
            .expect("Could not get character from line")
        {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Input data file contains bad character: {}", line),
        };

        let my_hand = match line
            .chars()
            .nth(2)
            .expect("Could not get character from line")
        {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("Input data file contains bad character: {}", line),
        };

        part_1_total_score += my_hand + 1;

        // Case for draw
        if my_hand == their_hand {
            part_1_total_score += DRAW;
        }
        // Case for win
        else if my_hand == (their_hand + 1) % 3 {
            part_1_total_score += WIN;
        }
        // Case for loose
        else {
            part_1_total_score += LOOSE;
        }

        let outcome = match line
            .chars()
            .nth(2)
            .expect(format!("Input data file contains bad character: {}", line).as_str())
        {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Input data file contains bad character: {}", line),
        };

        part_2_total_score += outcome;

        // Win condition
        if outcome == 6 {
            part_2_total_score += ((their_hand + 1) % 3) + 1;
        }

        // Draw condition
        if outcome == 3 {
            part_2_total_score += their_hand + 1;
        }

        if outcome == 0 {
            let new_val = {
                if their_hand - 1 <= -1 {
                    3
                } else {
                    (their_hand - 1) + 1
                }
            };

            part_2_total_score += new_val;
        }
    }

    (part_1_total_score, part_2_total_score)
}
