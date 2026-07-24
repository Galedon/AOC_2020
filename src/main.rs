use std::collections::{HashSet};
use crate::solutions::read_lines_to_str;

mod solutions;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut data = read_lines_to_str("input/8.txt")?;

    let (_, res) = one_Run_8(&mut data)?;
    println!("res a = {:?}", res);
    for i in 0 .. data.len(){
    // for  (i, line) in data.iter_mut().enumerate(){
        let line_orig = data[i].clone();
        let parts = data[i].split_whitespace().collect::<Vec< & str >>();
        match parts[0]{
            "jmp" => data[i] = format!("nop {}", parts[1]),
            "nop" => data[i] = format!("jmp {}", parts[1]),
            "acc" => continue,
            _ => panic!("invalid line input: {:?}", data[i])
        }
        let (succ, res) = one_Run_8(&mut data)?;
        if succ{println!("res b = {:?}", res); break}
        data[i] = line_orig;
        


    }





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

fn one_Run_8(data: &Vec<String>) -> Result<(bool,i32), Box<dyn std::error::Error>>{
    let mut visited = HashSet::new();
    let mut accumulator = 0;
    let mut i: i32 = 0;
    loop{
        if visited.contains( & i) {return Ok((false, accumulator))}
        if i == (data.len()-1) as i32{
            println ! ("success!!");
            return Ok((true, accumulator))
        }
        visited.insert(i);
        let line = & data[i as usize];
        let parts = line.split_whitespace().collect::<Vec < & str > > ();
        // println!("{:?}", parts);
        match parts[0] {
            "acc" => { accumulator += parts[1].parse::<i32 > () ?; i += 1; },
            "jmp" => i += parts[1].parse::< i32 > () ?,
            "nop" => i += 1,
            _ => panic ! ("invalid input: {}", parts[0])
        }
    }

}