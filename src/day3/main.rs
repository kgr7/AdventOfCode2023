use std::collections::HashMap;
use std::fs;

fn parse_input() -> Vec<char> {
    let loc = "../../inputs/day3.txt";
    let contents = fs::read_to_string(loc).expect("Could not read from file.");
    let chars = contents
        .chars()
        .filter(|&x| x != 0xA as char) // newline
        .collect::<Vec<char>>();

    chars
}

fn check_for_number(idx: usize, numbers: &Vec<char>, chars: &Vec<char>) -> usize {
    if idx >= chars.len() {
        return 0;
    }

    if numbers.contains(&chars[idx]) {
        return idx;
    }
    0
}

fn match_number(start_idx: usize, numbers: &Vec<char>, chars: &Vec<char>) -> (usize, usize) {
    let mut forward_idx = start_idx;
    let mut backward_idx = start_idx;

    while numbers.contains(&chars[backward_idx]) {
        backward_idx -= 1;
    }

    while forward_idx <= chars.len() && numbers.contains(&chars[forward_idx]) {
        forward_idx += 1;
    }

    (backward_idx + 1, forward_idx)
}

fn join_to_num(start: usize, end: usize, chars: &Vec<char>) -> i32 {
    let mut res: String = String::new();
    let slice = chars[start..end].into_iter();
    for c in slice {
        res.push(*c);
    }
    res.parse::<i32>().unwrap()
}

pub async fn part_1() {
    let numbers: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let chars: Vec<char> = parse_input();
    let mut total = 0;
    let mut seen: Vec<(usize, usize)> = Vec::new();

    let mut gear_map: HashMap<usize, Vec<i32>> = HashMap::new();

    for i in 0..chars.len() {
        if chars[i] != 0x2E as char && !numbers.contains(&chars[i]) {
            let number_checks = vec![
                check_for_number(i - 141, &numbers, &chars),
                check_for_number(i - 140, &numbers, &chars),
                check_for_number(i - 139, &numbers, &chars),
                check_for_number(i - 1, &numbers, &chars),
                check_for_number(i + 1, &numbers, &chars),
                check_for_number(i + 139, &numbers, &chars),
                check_for_number(i + 140, &numbers, &chars),
                check_for_number(i + 141, &numbers, &chars),
            ];

            for check_idx in number_checks.iter().filter(|&x| *x > 0 as usize) {
                let (start, end) = match_number(*check_idx, &numbers, &chars);
                if seen.contains(&(start, end)) {
                    continue;
                }
                seen.push((start, end));
                let final_num = join_to_num(start, end, &chars);
                total += final_num;

                // matches for asterisk (*)
                if chars[i] == 0x2A as char {
                    gear_map.entry(i).or_insert(Vec::new()).push(final_num);
                }
            }
        }
    }

    let mut total_products = 0;
    for (asterisk_idx, values) in &gear_map {
        if values.len() < 2 {
            continue;
        }
        let product: i32 = values.iter().product();
        total_products += product;
    }

    println!("Total products: {}", total_products);
    println!("Total is: {}", total);
}
