//! # Day4 Supply Stacks
//! I've decided to actually document this one because it has a few moving parts.
//!
//! I will use char vecs for modelling the stacks of items.
//! These will be contained within a vec.
//! This vec will be generated and filled at the beginning of the program in
//! a function that reads the input file.
//!
//! The instructions for moving around the items shall be converted to a struct
//! containing the important numbers for the movement.

use itertools::Itertools;

/// A struct defining a single operation moving 1 or more crates from one stack to another
#[derive(Debug)]
struct Move {
    /// The starting Stack
    origin: usize,
    /// The Destination Stack
    target: usize,
    /// The number of crates to move from start to destination
    crates: u32,
}

impl Move {
    fn new(origin: usize, target: usize, crates: u32) -> Self {
        Move {
            origin,
            target,
            crates,
        }
    }
}

pub fn process_data(data_lines: Vec<String>) -> (String, String) {
    // Split the input file into definitions for the stack and movements.
    let (stack_def, moves_def) = data_lines.split(|l| l == "").next_tuple().unwrap();

    // Read in the input data
    let mut stacks_vec = generate_stacks_vec(stack_def);
    let moves_vec = generate_moves_vec(moves_def);

    // Run the the process
    for m in moves_vec {
        for _ in 0..m.crates {
            if let Some(c) = stacks_vec[m.origin - 1].pop() {
                stacks_vec[m.target - 1].push(c)
            }
        }
    }

    // Collect the outputs
    let mut part1_output_vec = vec![];
    for stack in stacks_vec {
        part1_output_vec.push(stack[stack.len() - 1]);
    }

    let part1_output_str: String = part1_output_vec.into_iter().collect();

    (part1_output_str, "0".to_string())
}

/// Generate the vectors defining the stack of crates.
///
/// Takes in the lines of the input file defining the stacks start conditions.
/// Returns the fully formed and populated stacks.
fn generate_stacks_vec(stacks_ref: &[String]) -> Vec<Vec<char>> {
    // Define a vector of the correct size
    let num_stacks = get_number_of_stacks(&stacks_ref.last().unwrap());
    let mut stacks_vec = vec![vec![]; num_stacks];

    // Remove the last line containing the labels at the bottom of the stack definition
    let mut stacks_without_labels: Vec<&String> = stacks_ref
        .split_last()
        .unwrap()
        .1
        .into_iter()
        .rev()
        .collect();

    // Fill the stacks from the state given in stacks_ref
    // For each line, iterate through the letters (found by indexing every 4th point after 1)
    // If there is a character in the spot corresponding to a stack, place the character in the stack.
    for line in stacks_without_labels {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks_vec[i].push(c)
                }
            })
    }

    stacks_vec
}

/// Get the number of stacks from the labels in the last line of the stack definition
fn get_number_of_stacks(stack_labels_str: &str) -> usize {
    stack_labels_str
        .split(" ")
        .filter(|c| c != &"")
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

/// Generate the vector containing the moves to be made on the stacks.
///
/// Takes in the part of the input file defining the moves.
/// Returns a vec of `Move`s to be run on the stacks.
fn generate_moves_vec(moves_def: &[String]) -> Vec<Move> {
    moves_def
        .iter()
        .map(|l| {
            let l_chars: Vec<char> = l.chars().collect();
            Move::new(
                l_chars[12] as usize - 48,
                l_chars[17] as usize - 48,
                l_chars[5] as u32 - 48,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_stacks() {
        let test_input = " 1   2   3   4 ";
        let expected_output = 4usize;

        let actual_output = get_number_of_stacks(test_input);

        assert_eq!(expected_output, actual_output);
    }
}
