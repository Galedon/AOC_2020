// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let data = read_lines_to_str("input/5.txt")?;
    let mut id_max = 0;
    for line in data{
        let row_s = &line[..7].replace("B", "1").replace("F", "0");
        let col_s =&line[7..].replace("L", "0").replace("R", "1");

        let row = i32::from_str_radix(&row_s, 2).unwrap();
        let col = i32::from_str_radix(&col_s, 2).unwrap();

        let id = row * 8 + col;
        if id > id_max{
            id_max = id;
        }
    }
    println!("{}", id_max);

    Ok(())
}


fn read_lines_to_str(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<_>, _>>()
}


// fn read_file_to_int(path: &str) -> Result<Vec<i32>, std::io::Error>{
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let mut data: Vec<i32> = Vec::new();
//     for line_res in reader.lines() {
//         let line = line_res?;
//         data.push(line.parse().unwrap());
//     }
//     Ok(data)
// }

// fn get_day_1() -> Result<(), Box<dyn std::error::Error>> {
//     let data = read_file_to_int("input/1.txt")?;
//     let mut a = 0;
//     let mut b = 0;
//
//     for &x in &data {
//         for &z in &data {
//             if x + z == 2020 {
//                 a = x * z;
//             }
//             for &y in &data {
//                 if x + y + z == 2020 {
//                     b = x * y * z;
//                 }
//             }
//         }
//     }
//
//     println!("a = {}, b = {}", a, b);
//     Ok(())
// }
// fn get_day_2() -> Result<(), Box<dyn std::error::Error>> {
//     let mut n_valid_a = 0;
//     let mut n_valid_b = 0;
//     let data = read_file_to_str("input/2.txt")?;
//     for line in data.iter(){
//         let (policy, password) = line.split_once(':').ok_or("split")?;
//         let (range, letter) = policy.trim().split_once(' ').ok_or("split")?;
//         let (min_s, max_s) = range.split_once('-').ok_or("split")?;
//
//         let p1: usize = min_s.parse()?;
//         let p2: usize = max_s.parse()?;
//         let ch = letter.trim().chars().next().ok_or("char")?;
//
//         let count = password.chars().filter(|c| *c == ch).count();
//         if (p1..=p2).contains(&count) {
//             n_valid_a += 1;
//         }
//
//         if (password.chars().nth(p1) == Some(ch))^(password.chars().nth(p2) == Some(ch))
//         {
//             n_valid_b += 1;
//         }
//     }
//     println!("walid passwords a: {}", n_valid_a);
//     println!("walid passwords b: {}", n_valid_b);
//     Ok(())
// }

// fn get_day3() -> Result<(), Box<dyn std::error::Error>> {
//     let data = read_file_to_str("input/3.txt")?;
//     let mut hill: Vec<Vec<char>> = Vec::new();
//     for row in data{
//         hill.push(row.chars().collect());
//     }
//     let slope_vec = vec![
//         (1,1),
//         (3,1),
//         (5,1),
//         (7,1),
//         (1,2),
//     ];
//     let x_base = hill[0].len();
//     let mut res:i64 = 1;
//     for (slope_x, slope_y) in slope_vec {
//         let mut x = 0;
//         let mut y = 0;
//         let mut n_trees = 0;
//         while y < hill.len() {
//             if hill[y][x % x_base] == '#' { n_trees += 1 }
//             y += slope_y;
//             x += slope_x;
//         }
//         if (slope_x, slope_y) == (3,1){println!("res a = {}", n_trees);}
//         res *= n_trees;
//     }
//     println!("res b = {}", res);
//     Ok(())
// }
//
// fn get_day4() -> Result<(), Box<dyn std::error::Error>> {
//     let mut all_fields: HashMap<&str, fn(&str) -> bool> = HashMap::new();
//     all_fields.insert("byr", |x| {
//         if x.len() != 4 {return false};
//         for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
//         let n = x.parse::<i32>().unwrap();
//         if n < 1920 || n > 2002 {return false};
//         true
//     });
//     all_fields.insert("iyr", |x| {
//         if x.len() != 4 {return false};
//         for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
//         let n = x.parse::<i32>().unwrap();
//         if n < 2010 || n > 2020 {return false};
//         true
//     });
//     all_fields.insert("eyr", |x| {
//         if x.len() != 4 {return false};
//         for ch in x.chars() {if !ch.is_ascii_digit(){return false};}
//         let n = x.parse::<i32>().unwrap();
//         if n < 2020 || n > 2030 {return false};
//         true
//     });
//     all_fields.insert("hgt", |x| {
//         if !(x.ends_with("cm") || x.ends_with("in")) {return false};
//         let (val_s, unit) = x.split_at(x.len() - 2);
//         if val_s.len() == 0 {return false};
//         let val = val_s.parse::<i32>().unwrap();
//         if unit == "cm" {
//             if val < 150 || val > 193 {return false};
//             return true
//         }
//         if unit == "in"{
//             if (val < 59) || (val > 76) {return false};
//             return true
//         }
//         panic!("This should never have happened!!");
//     });
//     all_fields.insert("hcl", |x| {
//         if !x.starts_with("#") {return false};
//         if x.len() != 7 {return false};
//         for ch in x[1..].chars() {if !ch.is_ascii_hexdigit() {return false};}
//         true
//     });
//     all_fields.insert("ecl", |x| {
//         matches!(x, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
//     });
//     all_fields.insert("pid", |x| {
//         if x.len() != 9 {return false};
//         for ch in x.chars() {
//             if !ch.is_ascii_digit() {return false};
//         }
//         true
//     });
//
//     let mut valid_passports_a = 0;
//     let mut valid_passports_b = 0;
//     let lines = read_lines_to_str("input/4.txt")?;
//     let mut valid_fields: HashMap<&str, &str> = HashMap::new();
//     for line in &lines {
//         if line.len() == 0 {
//             if valid_fields.len() == 0 {continue}
//             let mut valid_a = true;
//             for k in all_fields.keys() {
//                 if *k == "cid"{continue}
//                 if !valid_fields.contains_key(k) {
//                     valid_a = false;
//                     break;
//                 }
//             }
//             if valid_a { valid_passports_a += 1;}
//
//             let mut valid_b = true;
//             for (k, val) in &valid_fields{
//                 if *k == "cid"{continue}
//                 // println!("{}: {}", k, val);
//                 if all_fields[k](*val){
//                     continue
//                 }
//                 valid_b = false;
//             }
//             if valid_b & valid_a { valid_passports_b += 1;}
//
//             valid_fields = HashMap::new();
//             continue;
//         }
//
//         let logs = line.trim().split_whitespace();
//         for log in logs {
//             let (mut key, mut val) = log.split_once(":").ok_or("no :")?;
//             key = key.trim();
//             val = val.trim();
//             valid_fields.insert(key, val);
//         }
//     }
//     println!("Valid passports_a: {}", valid_passports_a);
//     println!("Valid passports_b: {}", valid_passports_b);
//     Ok(())
// }