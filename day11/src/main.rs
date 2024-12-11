use std::collections::HashMap;

use ::aoc::aoc::AocResult;

// Key: Stone number
// Value: How many stones of that number are there
type StonesMap = HashMap<u64, u64>;

fn read_input(s: &str, initial_blink_count: u64) -> AocResult<StonesMap> {
    let mut sm = StonesMap::new();

    for ss in s.split_ascii_whitespace() {
        let stone_number: u64 = ss.parse().unwrap();
        sm.entry(stone_number).and_modify(|x| *x += 1).or_insert(1);
    }

    Ok(sm)
}

fn main() {
    let input_str = std::fs::read_to_string("input/day11.txt").expect("The file couldnt be read");

    let num_blinks = 75;

    let mut sm = read_input(&input_str, num_blinks).unwrap();

    let num_stones = blink_stones_and_count(&mut sm, num_blinks);
    //println!("{:?}", sm);
    println!(
        "Part 1: Num stones after blinking {} times: {:?}",
        num_blinks, num_stones
    );
}

fn blink_stones_once(sm: &mut StonesMap) -> StonesMap {
    let mut return_sm = StonesMap::new();

    for (stone_num, stone_count) in sm {
        if *stone_num == 0 {
            return_sm
                .entry(1)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);

        //println!("1");
        } else if stone_num.to_string().len() % 2 == 0 {
            let (left_digits, right_digits) = split_digits(&stone_num);

            return_sm
                .entry(left_digits)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);

            return_sm
                .entry(right_digits)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);

        //println!("split");
        } else {
            return_sm
                .entry(stone_num * 2024)
                .and_modify(|x| *x += *stone_count)
                .or_insert(*stone_count);
            //println!("other. new stone; {}", stone.num );
        }
    }

    return_sm
}

fn blink_stones_and_count(sm: &mut StonesMap, num_blinks: u64) -> u64 {
    let mut count = 0;

    for _ in 0..num_blinks {
        *sm = blink_stones_once(sm);
    }

    for v in sm.values() {
        count += v;
    }

    count
}

fn split_digits(num: &u64) -> (u64, u64) {
    let digits_as_str = num.to_string();
    let num_digits = digits_as_str.len();

    let (left_digits_str, right_digits_str) = digits_as_str.split_at(num_digits / 2);

    (
        left_digits_str.parse().unwrap(),
        right_digits_str.parse().unwrap(),
    )
}
