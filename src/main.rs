use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // get_day_1();
    get_day_2();

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

fn read_file_to_int(path: &str) -> Result<Vec<i32>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data: Vec<i32> = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        data.push(line.parse().unwrap());
    }
    Ok(data)
}

fn get_day_1() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("a = {}, b = {}", a, b);
    Ok(())
}
fn get_day_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut n_valid_a = 0;
    let mut n_valid_b = 0;
    let data = read_file_to_string("input/2.txt")?;
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
    println!("walid passwords a: {}", n_valid_a);
    println!("walid passwords b: {}", n_valid_b);
    Ok(())

}