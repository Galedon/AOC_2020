use std::collections::{HashMap, HashSet, VecDeque};
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
    let data = read_lines_to_str("input/7.txt")?;

    let mut  bags_conditions:HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut is_in_map = HashMap::new();
    for line in data{
        let parts = line.split("contain").collect::<Vec<&str>>();

        let outer = parts[0]
            .trim()
            .trim_end_matches("bag")
            .trim_end_matches("bags")
            .trim()
            .to_string();
        bags_conditions.insert(outer.clone(), HashMap::new());
        for mut inner in parts[1].split(","){
            if inner.contains("no other"){continue};
            let inner_val = inner.split_whitespace()
                .next().ok_or("inner is empty")?
                .parse::<i32>()?;
            // println!("inner: {}, inner val: {}", inner, inner_val);
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

            bags_conditions
                        .get_mut(&outer).ok_or("problem with key reading")?
                        .insert(inner.to_string(), inner_val);

            if !is_in_map.contains_key(inner) {is_in_map.insert(inner.to_string(), HashSet::new());}

            is_in_map.get_mut(inner).ok_or("is_in generation problem")?
                .insert(outer.clone());
        }
    }

    let mut possibles = HashSet::new();
    possibles.insert("shiny gold".to_string());
    let mut l_old = possibles.len();
    loop {
        for key in possibles.clone(){

            if let Some(is_in_list) = is_in_map.get(&key) {
                for is_in in is_in_list {
                    possibles.insert(is_in.to_string());
                }
            }
        }
        if possibles.len() == l_old {break}
        l_old = possibles.len();
    }
    println!("res a: {}", possibles.len()-1);

    let mut to_calc:VecDeque<InnerBag> = VecDeque::new();
    to_calc.push_back(InnerBag::from("shiny gold".to_string(), 1));
    let mut bags  = 0;
    while let Some(inner_back) = to_calc.pop_front(){
        for (key, val) in &bags_conditions[&inner_back.color]{
            to_calc.push_back(InnerBag::from (key.to_string(), *val*inner_back.count));
            bags += *val*inner_back.count;
        }
    }
    println!("res b: {}", bags);








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
