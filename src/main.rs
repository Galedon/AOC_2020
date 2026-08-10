use std::collections::{HashSet};
use crate::solutions::{read_lines_to_str, read_file_to_int, read_file_to_int_64};

mod solutions;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut data = read_file_to_int("input/10.txt")?;
    data.push(0);
    data.sort();

    let mut one_jolt = 0;
    let mut three_jolt = 0;
    for i in 0..data.len()-1{
        if data[i+1] - data[i] == 1{ one_jolt += 1; }
        if data[i+1] - data[i] == 3{ three_jolt += 1; }
        if data[i+1] - data[i] > 3{ panic!("too much jolt diff: {}, i = {}", data[i+1] - data[i],i) }
    }
    three_jolt += 1;
    println!("{}", one_jolt * three_jolt);



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

    // println!("day 9:");
    // println!("{}", solutions::get_day_9()?);
    // println!();

    Ok(())
}

