// https://adventofcode.com/2024/day/2

use std::{fs::File, io::{self, BufRead}};

fn main() {
    let reports = read_reports("input/day02.txt");
    let safe_report_count = count_safe_reports(&reports);
    println!("Num safe reports: {:?}", safe_report_count);

    let safe_report_count = count_safe_reports_with_dampener(&reports);
    println!("Num safe reports: {:?}", safe_report_count);
}

fn count_safe_reports(reports: &Vec<Vec<u32>>) -> u32 {
    let mut safe_report_count = 0;

    for report in reports {
        let is_safe = is_report_safe(report);
        println!("{:?}", is_safe);
	if is_safe {
	    safe_report_count += 1;
	}
    }

    safe_report_count
}

fn count_safe_reports_with_dampener(reports: &Vec<Vec<u32>>) -> u32 {
    let mut safe_report_count = 0;

    for report in reports {
        let is_safe = is_report_safe(report);

	if is_safe {
	    safe_report_count += 1;
	} else {
	    for idx_to_remove in 0..report.len() {
		let mut sliced_report = report.clone();
		sliced_report.remove(idx_to_remove);
		if is_report_safe(&sliced_report) {
		    safe_report_count += 1;
		    break;
		}
	    }
	}
    }

    safe_report_count
}

fn read_reports(filename: &str) -> Vec<Vec<u32>> {
    let mut reports: Vec<Vec<u32>> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let lines = reader.lines();
    for line in lines {
        reports.push(convert_line_to_report(&(line.unwrap())));
    }

    reports
}

fn convert_line_to_report(line: &str) -> Vec<u32> {
    let mut ret_vec: Vec<u32> = Vec::new();

    let split: Vec<&str> = line.split_whitespace().collect();

    for element in split {
        ret_vec.push(element.to_string().parse::<u32>().unwrap());
    }

    ret_vec
}

fn is_report_safe(report: &[u32]) -> bool {
    // calculate a 'distance_vector', then see if every entry has the same sign,
    // and if there are any values greater than 3

    let distance_vector: Vec<i32> = calc_distance_vector(report);

    let all_positive: bool = distance_vector.iter().all(|x| *x > 0);
    let all_negative: bool = distance_vector.iter().all(|x| *x < 0);

    let all_same_sign_and_nonzero: bool = all_positive || all_negative;
    let all_magnitude_less_than_or_equal_to_three: bool = distance_vector.iter().all(|x| (x.abs() <= 3));

    all_same_sign_and_nonzero && all_magnitude_less_than_or_equal_to_three
}

/// Returns a vector of the distance between subsequent numbers in the input vector
fn calc_distance_vector(report: &[u32]) -> Vec<i32> {
    let mut distance_vec = Vec::new();

    let mut last_num: i32 = i32::try_from(report[0]).unwrap();

    for cur_num in report[1..].iter() {
        let cur_num_i32 = i32::try_from(*cur_num).unwrap();
        let dist = cur_num_i32 - last_num;
        distance_vec.push(dist);
        last_num = cur_num_i32;
    }

    distance_vec
}

#[cfg(test)]
mod tests {
    // This is the example input provided in the problem
    static TEST_FILE: &str = "../../input/day02_ex01.txt";

    use super::*;

    #[test]
    fn test_sample_data() {
        let reports = read_reports(TEST_FILE);

    }
}
