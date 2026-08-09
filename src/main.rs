use std::collections::{HashSet};
use crate::solutions::{read_lines_to_str, read_file_to_int, read_file_to_int_64};

mod solutions;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = read_file_to_int_64("input/9.txt")?;


    // println!("day 1:");
    // println!("{}", solutions::get_day_1()?);
    // println!();
    //
    // println!("day 2:");
    // println!("{}", solutions::get_day_2()?);
    // println!();
    //
    // println!("day 3:");
    // println!("{}", solutions::get_day_3()?);
    // println!();
    //
    // println!("day 4:");
    // println!("{}", solutions::get_day_4()?);
    // println!();
    //
    // println!("day 5:");
    // println!("{}", solutions::get_day_5()?);
    // println!();

    // println!("day 6:");
    // println!("{}", solutions::get_day_6()?);
    // println!();

    // println!("day 7:");
    // println!("{}", solutions::get_day_7()?);
    // println!();

    // println!("day 8:");
    // println!("{}", solutions::get_day_8()?);
    // println!();

    println!("day 9:");
    println!("{}", solutions::get_day_9()?);
    println!();

    Ok(())
}

