use std::error::Error;

pub async fn part_1() -> Result<(), Box<dyn Error>> {
    let input = crate::shared::get_input::get_input_with_cookie("https://adventofcode.com/2023/day/1/input").await?;
    let mut total: i32 = 0;
    let words = input.split("\n"); 
    for word in words {
        total += i32::from(find_calibration_value(&word));
    }

    println!("Total is: {}", total);
    Ok(())
}

fn find_calibration_value(word: &str) -> i8 {
    let mut res: i8 = 0;
    let start = 0;
    let end = word.chars().count();

    'first: for x in start..end {
        let char = word.chars().nth(x).expect("Couldn't unwrap char");
        if char.is_digit(10) {
            res += i8::try_from(char.to_digit(10).unwrap_or(0) * 10).unwrap();
            break 'first;
        }
    }

    'last: for x in (start..end).rev() {
        let char = word.chars().nth(x).expect("Couldn't unwrap char");
        if char.is_digit(10) {
            res += i8::try_from(char.to_digit(10).unwrap_or(0)).unwrap();
            break 'last;
        }
    }

    res
}

pub async fn part_2() -> Result<(), Box<dyn Error>> {
    println!("Does this work??");
    Ok(())
}
