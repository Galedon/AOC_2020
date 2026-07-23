use std::collections::{HashMap, HashSet};
use crate::solutions::read_lines_to_str;

mod solutions;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = read_lines_to_str("input/7.txt")?;
    let mut is_in_map = HashMap::new();
    for line in data{
        let parts = line.split("contain").collect::<Vec<&str>>();

        let outer = parts[0]
            .trim()
            .trim_end_matches("bag")
            .trim_end_matches("bags")
            .trim()
            .to_string();

        for mut inner in parts[1].split(","){
            if inner.contains("no other"){continue};
            let inner_string = inner.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");
            inner = &inner_string;
            inner = inner
                .trim()
                .trim_end_matches(".")
                .trim_end_matches("bag")
                .trim_end_matches("bags")
                .trim();


            if !is_in_map.contains_key(inner) {is_in_map.insert(inner.to_string(), HashSet::new());}

            is_in_map.get_mut(inner).ok_or("is_in generation problem")?
                .insert(outer.clone());
        }
    }

    let mut posibles = HashSet::new();
    posibles.insert("shiny gold".to_string());
    let mut l_old = posibles.len();
    loop {
        for key in posibles.clone(){

            if let Some(is_in_list) = is_in_map.get(&key) {
                for is_in in is_in_list {
                    posibles.insert(is_in.to_string());
                }
            }
        }
        if posibles.len() == l_old {break}
        l_old = posibles.len();
    }
    println!("{}", posibles.len()-1);






    // let mut bags_conditions:HashMap<String, HashMap<String, i32>> = HashMap::new();
    // for line in data {
    //     let parts = line.split("contain").collect::<Vec<&str>>();
    //
    //     let mut key = parts[0].to_string();
    //     key = key[..key.len()-5].to_string();
    //     bags_conditions.insert(key.clone(), HashMap::new());
    //
    //     let val_parts = parts[1].split(",").collect::<Vec<&str>>();
    //     for mut part in val_parts {
    //         part = part.trim();
    //         let (mut val, typ) = part.split_once(" ").ok_or("val_parts are problematic")?;
    //         if val == "no" {
    //             // val = "0";
    //             continue;
    //         }
    //         bags_conditions
    //             .get_mut(&key).ok_or("problem with key reading")?
    //             .insert(typ.to_string(), val.to_string().parse()?);
    //     }
    //
    // }
    // println!("{:?}", bags_conditions);













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

    // println!("day 6:");
    // println!("{}", solutions::get_day_6()?);

    Ok(())
}


