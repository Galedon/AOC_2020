use std::collections::HashSet;

mod solutions;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = solutions::read_lines_to_str("input/6.txt")?;

    let mut yes_sum = 0;
    let mut yes_set = HashSet::new();
    for line in data{
        if line.is_empty(){
            yes_sum += yes_set.len();
            yes_set.clear();
            continue;
        }
        for ch in line.chars(){
            yes_set.insert(ch);
        }
    }
    println!("{}", yes_sum);






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
    
    Ok(())
}

