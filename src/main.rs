use std::collections::{HashMap, HashSet};
use crate::solutions::read_lines_to_str;

mod solutions;
#[derive(Debug)]
struct InnerBag {
    color: String,
    count: i32,
}
impl InnerBag {
    fn from (color: String, count: i32) -> InnerBag {
        InnerBag {color, count}
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = read_lines_to_str("input/8.txt")?;
    let mut visited = HashSet::new();
    let mut accumulator = 0;
    let mut i: i32 = 0;
    loop{
        if visited.contains(&i) {break;}
        visited.insert(i);
        let line = &data[i as usize];
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        // println!("{:?}", parts);
        match parts[0] {
            "acc" => { accumulator += parts[1].parse::<i32>()?; i += 1; },
            "jmp" => i += parts[1].parse::<i32>()?,
            _ =>  i += 1
        }
    }
    println!("accumulator: {}", accumulator);







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

    Ok(())
}
