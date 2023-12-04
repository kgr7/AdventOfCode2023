use std::collections:: HashMap;
use std::error::Error;

pub async fn part_1() -> Result<(), Box<dyn Error>> {
    let input = crate::shared::get_input::get_input_with_cookie("https://adventofcode.com/2023/day/1/input").await?;
    let mut total: i32 = 0;
    let words = input.split("\n"); 
    
    for word in words {
        total += i32::from(find_calibration_value(&word));
    }

    println!("Part1: total is: {}", total);
    Ok(())
}

fn find_calibration_value(word: &str) -> i8 {
    let mut res: i8 = 0;
    let start = 0;

    'first: for x in 0..word.chars().count() {
        let char = word.chars().nth(x).expect("Couldn't unwrap char");
        if char.is_digit(10) {
            res += i8::try_from(char.to_digit(10).unwrap_or(0) * 10).unwrap();
            break 'first;
        }
    }

    'last: for x in (0..word.chars().count()).rev() {
        let char = word.chars().nth(x).expect("Couldn't unwrap char");
        if char.is_digit(10) {
            res += i8::try_from(char.to_digit(10).unwrap_or(0)).unwrap();
            break 'last;
        }
    }

    res
}

fn find_calibration_value_text(words: Vec<&str>) -> i32 {
    let mut total: i32 = 0;
    let targets = [
        "one", "1",
        "two", "2",
        "three", "3",
        "four", "4",
        "five", "5",
        "six", "6",
        "seven", "7",
        "eight", "8",
        "nine", "9"
    ];

    for word in words {
        let mut first = None;
        let word_len = word.len();
        if word_len == 0 {
            continue;
        }
        'forward: for i in 0..word.len() {
            for (j, t) in targets.iter().enumerate() {
                if i + t.len() > word_len {
                    continue;
                }
                if **t == word[i..i + t.len()] {
                    first = Some((j / 2) + 1);
                    break 'forward;
                }
            }
        }
        let Some(first) = first else {
            panic!("Invalid");
        };
        let mut last = None;
        'backward: for i in (0..word.len()).rev() {
            for (j, t) in targets.iter().enumerate() {
                if i + t.len() > word_len {
                    continue;
                }
                if **t == word[i..i + t.len()] {
                    last = Some((j / 2) + 1);
                    break 'backward;
                }
            }
        }
        let Some(last) = last else {
            panic!("Invalid");
        };
        total += 10 * first as i32 + last as i32;
    }

    total
}

pub async fn part_2() -> Result<(), Box<dyn Error>> {
    let input = crate::shared::get_input::get_input_with_cookie("https://adventofcode.com/2023/day/1/input").await?;
    let words = input.split("\n");
    let words_vec = words.collect::<Vec<_>>();

    let total = find_calibration_value_text(words_vec);
    println!("Part2: total is: {}", total);
    Ok(())
}
