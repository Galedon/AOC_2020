use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    get_day3();



    Ok(())
}

fn read_file_to_string(path: &str) -> Result<Vec<String>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        data.push(line);
    }
    Ok(data)
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
//     let data = read_file_to_string("input/2.txt")?;
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

fn get_day3() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file_to_string("input/3.txt")?;
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
    let mut res:i64 = 1;
    for (slope_x, slope_y) in slope_vec {
        let mut x = 0;
        let mut y = 0;
        let mut n_trees = 0;
        while y < hill.len() {
            if hill[y][x % x_base] == '#' { n_trees += 1 }
            y += slope_y;
            x += slope_x;
        }
        if (slope_x, slope_y) == (3,1){println!("res a = {}", n_trees);}
        res *= n_trees;
    }
    println!("res b = {}", res);
    Ok(())
}