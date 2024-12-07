// https://adventofcode.com/2024/day/2

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Clone, Debug)]
struct Equation {
    total: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, PartialEq)]
enum Error {
    ParseError,
    FileOpenError,
}

fn main() {
    let input_filename = "input/day07.txt";

    // Part 1
    let reader = get_calibration_equation_reader(input_filename).expect("Couldn't read input file");
    let use_concat_operator = false;
    let sum_of_valid_equations = calc_sum_of_valid_equations(reader, use_concat_operator);
    println!(
        "Part 1: Sum of valid equations totals, not using concat operator: {}",
        sum_of_valid_equations
    );

    // Part 2
    let reader = get_calibration_equation_reader(input_filename).unwrap(); // just unwrap. It worked for part 1, it's not going to fail now...
    let use_concat_operator = true;
    let sum_of_valid_equations = calc_sum_of_valid_equations(reader, use_concat_operator);
    println!(
        "Part 2: Sum of valid equations totals, using concat operator: {}",
        sum_of_valid_equations
    );
}

/// Rather than read the entire input into memory at once, allow getting it line by line
fn get_calibration_equation_reader(filename: &str) -> Result<Lines<BufReader<File>>, Error> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(Error::FileOpenError),
    };

    Ok(io::BufReader::new(file).lines())
}

/// One line at a time - Read one line from the input, try to parse it, and see if its valid.
/// If it's a valid equation, add its total to the overall total
fn calc_sum_of_valid_equations(lines: Lines<BufReader<File>>, use_concat_operator: bool) -> u64 {
    let mut sum: u64 = 0;

    for line in lines {
        let eqn_str = line.expect("Couldn't read another line from the input file");
        let eqn = parse_equation(&eqn_str)
            .unwrap_or_else(|_| panic!("Couldn't parse equation {:?}", eqn_str));

        let is_valid_equation: bool = calc_is_valid_equation(&eqn, use_concat_operator);
        if is_valid_equation {
            sum += eqn.total;
        }
    }

    sum
}

/// Parse a string representing an equation, looking like:
/// 161011: 16 10 13
/// Into an Equation struct:
/// Equation { total: 161011, numbers: [16, 10, 13] }
fn parse_equation(eqn_str: &str) -> Result<Equation, Error> {
    let mut parts = eqn_str.split(": ");

    let total_str = match parts.next() {
        Some(s) => s,
        None => return Err(Error::ParseError),
    };

    let numbers_str = match parts.next() {
        Some(s) => s,
        None => return Err(Error::ParseError),
    };

    let my_total: u64 = match total_str.parse::<u64>() {
        Ok(t) => t,
        Err(_) => return Err(Error::ParseError),
    };

    let my_numbers = numbers_str
        .split_ascii_whitespace()
        .map(|x| {
            let parsed_u64 = my_parse_u64(x)?;
            Ok(parsed_u64)
        })
        .collect::<Result<Vec<u64>, Error>>()?;

    Ok(Equation {
        total: my_total,
        numbers: my_numbers,
    })
}

fn my_parse_u64(u64_str: &str) -> Result<u64, Error> {
    match u64_str.parse::<u64>() {
        Ok(u) => Ok(u),
        Err(_) => Err(Error::ParseError),
    }
}

/// Recursively replace the first two numbers in eqn.numbers with one of the
/// possible operators. Once at the base case, see if we got the right total
fn calc_is_valid_equation(eqn: &Equation, use_concat_operator: bool) -> bool {
    // recursive base case is if just one item is left in 'eqn.numbers'
    if eqn.numbers.len() == 1 {
        // Is eqn like this...?  Equation{ total: 42, numbers: [42] }
        return eqn.numbers[0] == eqn.total;
    }

    // Early quit... this recursive path can't work because its accumulation so
    // far is too big
    if eqn.numbers[0] > eqn.total {
        return false;
    }

    let mut remaining_numbers = Vec::new();
    remaining_numbers.extend_from_slice(&eqn.numbers[1..]);

    let mut sub_eq = Equation {
        total: eqn.total,
        numbers: remaining_numbers,
    };

    // Recurse with a multiplication
    // Replace the first two numbers in equation with the product of those numbers
    sub_eq.numbers[0] = eqn.numbers[0] * eqn.numbers[1];
    let mul_valid = calc_is_valid_equation(&sub_eq, use_concat_operator);  

    // Recurse with an addition
    // Replace the first two numbers in equation with the sum of those numbers
    sub_eq.numbers[0] = eqn.numbers[0] + eqn.numbers[1];
    let add_valid = calc_is_valid_equation(&sub_eq, use_concat_operator);

    // Recurse with a concatenation
    // Replace the first two numbers in equation with the concatenation of those numbers
    let mut concat_valid: bool = false;
    if use_concat_operator {
	sub_eq.numbers[0] = concat_u64(eqn.numbers[0], eqn.numbers[1]);
        concat_valid = calc_is_valid_equation(&sub_eq, use_concat_operator);
    }

    if mul_valid || add_valid || concat_valid {
        return true;
    }

    false
}

/// Concatenate two u64s. For example, concat_u64(12, 345) becomes 12345
fn concat_u64(a: u64, b: u64) -> u64 {
    // You remember your logarithms right?

    let mag_b = b.ilog10();

    // Left-shift a by the magnitued of b, plus one more
    let a_shifted = a * 10_u64.pow(mag_b + 1);

    a_shifted + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_weird_file_is_error() {
	let input_filename = "no_way_this_exists";
	let actual = get_calibration_equation_reader(input_filename).err().unwrap();

	assert_eq!(Error::FileOpenError, actual);
    }

    #[test]
    fn test_parse_bad_string_return_error() {
	let bad_string = "bad string";
	let actual = parse_equation(&bad_string).err().unwrap();

	assert_eq!(Error::ParseError, actual);
    }

    #[test]
    fn test_ex1_part1() {
	let eqn = Equation { total: 190, numbers: vec![10, 19] };
	let use_concat_operator = false;
	let expected = true;

	assert!(expected == calc_is_valid_equation(&eqn, use_concat_operator));
    }

    #[test]
    fn test_ex2_part1() {
	let eqn = Equation { total: 3267, numbers: vec![81, 40, 27] };
	let use_concat_operator = false;
	let expected = true;

	assert!(expected == calc_is_valid_equation(&eqn, use_concat_operator));
    }

    #[test]
    fn test_ex1_part2() {
	let eqn = Equation { total: 190, numbers: vec![10, 19] };
	let use_concat_operator = true;
	let expected = true;

	assert!(expected == calc_is_valid_equation(&eqn, use_concat_operator));
    }

    #[test]
    fn test_ex2_part2() {
	let eqn = Equation { total: 3267, numbers: vec![81, 40, 27] };
	let use_concat_operator = true;
	let expected = true;

	assert!(expected == calc_is_valid_equation(&eqn, use_concat_operator));
    }
}
