// https://adventofcode.com/2024/day/1

use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Error {
    ParseError,
}

fn main() {
    let input_str = fs::read_to_string("input/day01.txt").expect("Can't read input file!");

    let (vec_left, vec_right) = parse_input(&input_str).expect("Bad input string!");
    let right_frequencies: HashMap<u32, u32> = calc_frequencies(&vec_right);

    let total_dist = calc_distance(&vec_left, &vec_right);
    println!("Total distance: {:?}", total_dist);

    let similarity_score = calc_similarity_score(&vec_left, &right_frequencies);
    println!("Similarity score: {:?}", similarity_score);
}

/// Parse the problem input into a pair of sorted Vec<u32>s
///
/// # Example
/// 12 78
/// 56 34
///
/// Should return: ([12, 56], [34, 78])
fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut vec_left: Vec<u32> = Vec::new();
    let mut vec_right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();

	// The input should have 2 columns
	if split.len() != 2 {
	    return Err(Error::ParseError);
	}

        let left_num: u32 = parse_u32(split[0])?;
        let right_num: u32 = parse_u32(split[1])?;

        vec_left.push(left_num);
        vec_right.push(right_num);
    }

    vec_left.sort();
    vec_right.sort();

    Ok((vec_left, vec_right))
}

/// Try to parse a u32 from a string slice
fn parse_u32(u32str: &str) -> Result<u32, Error> {
    match u32str.parse::<u32>() {
	Ok(u) => Ok(u),
	Err(_) => Err(Error::ParseError),
    }
}

/// Return a HashMap that holds a count of how often numbers in a vector appear
///
/// # Arguments
///
/// * `vec` The vector to find frequencies in
fn calc_frequencies(vec: &[u32]) -> HashMap<u32, u32> {
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for item in vec {
        frequency_map
            .entry(*item)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    frequency_map
}

/// Calculate the aboslute distance beteween two iterables of numbers
///
/// It's the sum of the absolute difference of each element.
///
/// # Arguments
///
/// * `vec1` The first vector
/// * `vec2` The second vector
fn calc_distance(vec1: &[u32], vec2: &[u32]) -> u32 {
    let mut total_dist = 0;

    for i in vec1.iter().zip(vec2.iter()) {
        let abs_distance = abs_diff_u32(i.0, i.1);
        total_dist += abs_distance;
    }

    total_dist
}

/// Calculate the similarity score for a vector given a scorecard
///
/// # Arguments
///
/// * `vec` A vector of numbers
/// * `scorecard` How much each number is worth
fn calc_similarity_score(vec: &Vec<u32>, scorecard: &HashMap<u32, u32>) -> u32 {
    let mut total_similarity_score = 0;

    for i in vec {
        if scorecard.contains_key(i) {
            total_similarity_score += scorecard.get(i).unwrap() * i;
        }
    }

    total_similarity_score
}

/// Calculate the absolute difference between two u32s
///
/// # Arguments
///
/// * `first` The first u32
/// * `second` The second u32
fn abs_diff_u32(first: &u32, second: &u32) -> u32 {
    if first < second {
        second - first
    } else {
        first - second
    }
}

#[cfg(test)]
mod tests {
    // This is the example input provided in the problem
    static TEST_STR: &str = include_str!("../../input/day01_ex01.txt");

    use super::*;

    #[test]
    fn test_part1() {
        let (vec_left, vec_right) = parse_input(TEST_STR).expect("Parse error!");
        let frequency_map = calc_frequencies(&vec_right);

        assert_eq!(calc_distance(&vec_left, &vec_right), 11);
        assert_eq!(calc_similarity_score(&vec_left, &frequency_map), 31);
    }
}
