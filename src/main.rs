use std::collections::{HashSet};
use crate::solutions::{read_lines_to_str, read_file_to_int, read_file_to_int_64};

mod solutions;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = read_file_to_int_64("input/9.txt")?;
    let mut the_number = 0;
    let mut i = 26;

    loop{
        let mut ok = false;
        for j in i-25 .. i {
            for k in j+1 .. i {
                if data[j]+data[k] == data[i] {
                    ok = true;
                }
            }
        }
        // println!("i = {i}");
        if !ok {
            // println!("res_a = {}", data[i]);
            the_number = data[i];
            break}
        i += 1
    }
    println!("The number is {}", the_number);

    i = 0;
    let mut j_final = 0;
    loop {
        let mut ok = false;
        let mut sum = 0;

        let n = data.len();
        for j in 0 ..n {
            sum += data[i+j];
            if sum   == the_number{
                ok = true;
                j_final = j;
                break;
            }
            if (sum > the_number) | (i + j > n)
            {
                break;
            }
        }

        if ok {
            break;
        }
        i += 1;
    }
    let  min = data[i..i+j_final].iter().min().unwrap();
    let  max = data[i..i+j_final].iter().max().unwrap();

    println!("{}", min + max);


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

    Ok(())
}

