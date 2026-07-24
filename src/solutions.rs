use std::collections::{HashMap, VecDeque};
use crate::solutions::io::BufReader;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn read_lines_to_str(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<_>, _>>()
}


pub fn read_file_to_int(path: &str) -> Result<Vec<i32>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data: Vec<i32> = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        data.push(line.parse().unwrap());
    }
    Ok(data)
}

pub fn get_day_1() -> Result<String, Box<dyn std::error::Error>> {
    let data = read_file_to_int("input/1.txt")?;
    let mut a = 0;
    let mut b = 0;

    for &x in &data {
        for &z in &data {
            if x + z == 2020 {
                a = x * z;
            }
            for &y in &data {
                if x + y + z == 2020 {
                    b = x * y * z;
                }
            }
        }
    }

    Ok(format!("a = {}, b = {}", a, b))

}
pub fn get_day_2() -> Result<String, Box<dyn std::error::Error>> {
    let mut n_valid_a = 0;
    let mut n_valid_b = 0;
    let data = read_lines_to_str("input/2.txt")?;
    for line in data.iter(){
        let (policy, password) = line.split_once(':').ok_or("split")?;
        let (range, letter) = policy.trim().split_once(' ').ok_or("split")?;
        let (min_s, max_s) = range.split_once('-').ok_or("split")?;

        let p1: usize = min_s.parse()?;
        let p2: usize = max_s.parse()?;
        let ch = letter.trim().chars().next().ok_or("char")?;

        let count = password.chars().filter(|c| *c == ch).count();
        if (p1..=p2).contains(&count) {
            n_valid_a += 1;
        }

        if (password.chars().nth(p1) == Some(ch))^(password.chars().nth(p2) == Some(ch))
        {
            n_valid_b += 1;
        }
    }
    Ok(format!("walid passwords a: {} \nwalid passwords b: {}", n_valid_a, n_valid_b))
}

pub fn get_day_3() -> Result<String, Box<dyn std::error::Error>> {
    let data = read_lines_to_str("input/3.txt")?;
    let mut hill: Vec<Vec<char>> = Vec::new();
    for row in data{
        hill.push(row.chars().collect());
    }
    let slope_vec = vec![
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];
    let x_base = hill[0].len();
    let mut res_b:i64 = 1;
    let mut res_a = 0;
    for (slope_x, slope_y) in slope_vec {
        let mut x = 0;
        let mut y = 0;
        let mut n_trees = 0;
        while y < hill.len() {
            if hill[y][x % x_base] == '#' { n_trees += 1 }
            y += slope_y;
            x += slope_x;
        }
        if (slope_x, slope_y) == (3,1){res_a = n_trees;}
        res_b *= n_trees;
    }

    Ok(format!("res a = {}, res b = {}", res_a, res_b))
}

pub fn get_day_4() -> Result<String, Box<dyn std::error::Error>> {
    let mut all_fields: HashMap<&str, fn(&str) -> bool> = HashMap::new();
    all_fields.insert("byr", |x| {
        if x.len() != 4 {return false};
        for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
        let n = x.parse::<i32>().unwrap();
        if n < 1920 || n > 2002 {return false};
        true
    });
    all_fields.insert("iyr", |x| {
        if x.len() != 4 {return false};
        for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
        let n = x.parse::<i32>().unwrap();
        if n < 2010 || n > 2020 {return false};
        true
    });
    all_fields.insert("eyr", |x| {
        if x.len() != 4 {return false};
        for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
        let n = x.parse::<i32>().unwrap();
        if n < 2020 || n > 2030 {return false};
        true
    });
    all_fields.insert("hgt", |x| {
        if !(x.ends_with("cm") || x.ends_with("in")) {return false};
        let (val_s, unit) = x.split_at(x.len() - 2);
        if val_s.len() == 0 {return false};
        let val = val_s.parse::<i32>().unwrap();
        if unit == "cm" {
            if val < 150 || val > 193 {return false};
            return true
        }
        if unit == "in"{
            if (val < 59) || (val > 76) {return false};
            return true
        }
        panic!("This should never have happened!!");
    });
    all_fields.insert("hcl", |x| {
        if !x.starts_with("#") {return false};
        if x.len() != 7 {return false};
        for ch in x[1..].chars() {if !ch.is_ascii_hexdigit() {return false};}
        true
    });
    all_fields.insert("ecl", |x| {
        matches!(x, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    });
    all_fields.insert("pid", |x| {
        if x.len() != 9 {return false};
        for ch in x.chars() {
            if !ch.is_ascii_digit() {return false};
        }
        true
    });

    let mut valid_passports_a = 0;
    let mut valid_passports_b = 0;
    let lines = read_lines_to_str("input/4.txt")?;
    let mut valid_fields: HashMap<&str, &str> = HashMap::new();
    for line in &lines {
        if line.len() == 0 {
            if valid_fields.len() == 0 {continue}
            let mut valid_a = true;
            for k in all_fields.keys() {
                if *k == "cid"{continue}
                if !valid_fields.contains_key(k) {
                    valid_a = false;
                    break;
                }
            }
            if valid_a { valid_passports_a += 1;}

            let mut valid_b = true;
            for (k, val) in &valid_fields{
                if *k == "cid"{continue}
                // eprintln!("{}: {}", k, val);
                if all_fields[k](*val){
                    continue
                }
                valid_b = false;
            }
            if valid_b & valid_a { valid_passports_b += 1;}

            valid_fields = HashMap::new();
            continue;
        }

        let logs = line.trim().split_whitespace();
        for log in logs {
            let (mut key, mut val) = log.split_once(":").ok_or("no :")?;
            key = key.trim();
            val = val.trim();
            valid_fields.insert(key, val);
        }
    }
    Ok(format!("Valid passports_a: {} \nValid passports_b: {}", valid_passports_a, valid_passports_b))
}

pub fn get_day_5()-> Result<String, Box<dyn std::error::Error>>{
    let data = read_lines_to_str("input/5.txt")?;
    let mut id_max = 0;
    let mut ids: HashSet<i32> = HashSet::new();
    for line in data{
        let row_s = &line[..7].replace("B", "1").replace("F", "0");
        let col_s =&line[7..].replace("L", "0").replace("R", "1");

        let row = i32::from_str_radix(&row_s, 2).unwrap();
        let col = i32::from_str_radix(&col_s, 2).unwrap();

        let id = row * 8 + col;
        ids.insert(id);
        if id > id_max{
            id_max = id;
        }
    }

    let mut res_b = 0;
    for i in  0..id_max {
        if !ids.contains(&i){
            if !ids.contains(&(i+1)){ continue; }
            if !ids.contains(&(i-1)){ continue; }
            res_b = i;
            break
        }
    }

    Ok(format!("max id = {}, seat = {}", id_max, res_b ))
}

pub fn get_day_6()-> Result<String, Box<dyn std::error::Error>>{
    let data = read_lines_to_str("input/6.txt")?;
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
    Ok(format!("res_a = {}, res_b = {}", yes_sum, yes_sum_b ))
}
///////
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
pub fn get_day_7()-> Result<String, Box<dyn std::error::Error>>{
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
    let res_a =  possibles.len()-1;
    

    let mut to_calc:VecDeque<InnerBag> = VecDeque::new();
    to_calc.push_back(InnerBag::from("shiny gold".to_string(), 1));
    let mut bags  = 0;
    while let Some(inner_back) = to_calc.pop_front(){
        for (key, val) in &bags_conditions[&inner_back.color]{
            to_calc.push_back(InnerBag::from (key.to_string(), *val*inner_back.count));
            bags += *val*inner_back.count;
        }
    }
    let res_b = bags;
    
    Ok(format!("res_a = {}, res_b = {}", res_a, res_b ))
}

///////////
fn one_Run_8(data: &Vec<String>) -> Result<(bool,i32), Box<dyn std::error::Error>>{
    let mut visited = HashSet::new();
    let mut accumulator = 0;
    let mut i: i32 = 0;
    loop{
        if visited.contains( & i) {return Ok((false, accumulator))}
        if i == (data.len()-1) as i32{
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

pub fn get_day_8() -> Result<String, Box<dyn std::error::Error>>{
    let mut data = read_lines_to_str("input/8.txt")?;
    let mut res_b = 0;
    let (_, res) = one_Run_8(&mut data)?;
    let res_a = res;
    for i in 0 .. data.len(){
        let line_orig = data[i].clone();
        let parts = data[i].split_whitespace().collect::<Vec< & str >>();
        match parts[0]{
            "jmp" => data[i] = format!("nop {}", parts[1]),
            "nop" => data[i] = format!("jmp {}", parts[1]),
            "acc" => continue,
            _ => panic!("invalid line input: {:?}", data[i])
        }
        let (succ, res) = one_Run_8(&mut data)?;
        if succ{res_b = res; break}
        data[i] = line_orig;
    }
    Ok(format!("res a = {}, res b = {}", res_a, res_b ))
}