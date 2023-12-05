use std::error::Error;

mod shared;
mod day1;
mod day2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let _1 = day1::main::part_1().await;
    // let _2 = day1::main::part_2().await;
    let _1 = day2::main::part_1().await;
    Ok(())
}

