use std::collections::HashSet;

mod solutions;
fn main() -> Result<(), Box<dyn std::error::Error>>{

    let data = solutions::read_lines_to_str("input/6.txt")?;
    let mut yes_sum = 0;
    let mut yes_sum_b = 0;
    let mut yes_set = HashSet::new();
    let mut yes_set_b = HashSet::new();
    let mut new_group = true;
    for line in data{
        if line.is_empty(){
            yes_sum += yes_set.len();
            yes_sum_b += yes_set_b.len();
            yes_set.clear();
            yes_set_b.clear();
            new_group = true;
            continue;
        }

        let mut yes_line = HashSet::new();
        for ch in line.chars(){
            yes_set.insert(ch);
            yes_line.insert(ch);
        }
        if new_group{
            yes_set_b = yes_line.clone();
            new_group = false;
            continue;
        }
        yes_set_b = yes_set_b.intersection(&yes_line).cloned().collect();
    }
    println!("{}", yes_sum);
    println!("{}", yes_sum_b);




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

