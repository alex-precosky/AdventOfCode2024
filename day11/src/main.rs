use std::{collections::LinkedList, str::FromStr};

use ::aoc::aoc::AocResult;
use aoc::aoc::AocError;

type StonesLinkedList = LinkedList<u64>;

fn read_input(s: &str) -> AocResult<StonesLinkedList> {
    let mut sll = StonesLinkedList::new();

    for ss in s.split_ascii_whitespace() {
        let stone_number: u64 = ss.parse().unwrap();
        sll.push_back(stone_number);
    }

    Ok(sll)
}

fn main() {
    let input_str =
        std::fs::read_to_string("input/day11_ex02.txt").expect("The file couldnt be read");
    let mut sll = read_input(&input_str).unwrap();

    let num_blinks = 75;
    let num_stones = blink_stones_and_count(&mut sll, num_blinks);
    //println!("{:?}", sll);
    println!(
        "Part 1: Num stones after blinking {} times: {:?}",
        num_blinks, num_stones
    );

}

fn blink_stones_and_count(sll: &mut StonesLinkedList, num_blinks: u64) -> usize {
    let mut i = 0;
    for _ in 0..num_blinks {
        *sll = blink_stones(sll);
	println!("{}, {}", i, sll.len());
	i += 1;
    }

    sll.len()
}

fn blink_stones(sll: &mut StonesLinkedList) -> StonesLinkedList {
    let mut return_sll = StonesLinkedList::new();

    for stone in sll.iter_mut() {
        if *stone == 0 {
	    return_sll.push_back(1);
        } else if stone.to_string().len() % 2 == 0 {
	    let (left_digits, right_digits) = split_digits(&stone);
	    return_sll.push_back(left_digits);
	    return_sll.push_back(right_digits);
        } else {
	    return_sll.push_back(*stone * 2024);
        }
    }

    return_sll
}

fn insert_at(sll: &mut StonesLinkedList, idx: usize, val: u64) {
    let mut tail = sll.split_off(idx);
    sll.push_back(val);
    sll.append(&mut tail);
}

fn split_digits(num: &u64) -> (u64, u64) {

    let digits_as_str = num.to_string();
    let num_digits = digits_as_str.len();

    let (left_digits_str, right_digits_str) = digits_as_str.split_at(num_digits / 2);

    (left_digits_str.parse().unwrap(), right_digits_str.parse().unwrap())
}



// TODO. Blink stones 75 times, one at a time. pop_back() them once done blinking each stone, and increment a counter
