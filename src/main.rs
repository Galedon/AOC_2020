use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // get_day_1();
    let mut n_valid = 0;
    let data = read_file_to_string("input/2.txt")?;
    for line in data.iter(){
        let parts: Vec<_> = line.split(":").collect();
        let policy = parts[0];
        let password = parts[1];
        let p_parts: Vec<_> = policy.split(" ").collect();
        let p_char = p_parts[1];
        let p_num = p_parts[0];
        let p_num_parts: Vec<_> = p_num.split("-").collect();
        let p_min: i32 = p_num_parts[0].parse().unwrap();
        let p_max: i32 = p_num_parts[1].parse().unwrap();
        let p_contain: i32 = password.chars()
            .filter(|c| *c == p_char.chars().next().unwrap())
            .count() as i32;

        if  (p_contain >= p_min)&&(p_contain <= p_max){n_valid += 1;}
    }
    println!("walid passwords: {}", n_valid);
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

fn get_day_1() ->  Result<(), Box<dyn std::error::Error>>{
    let data = read_file_to_int("input/1.txt")?;
    let mut a = 0;
    let mut b = 0;
    data.iter().for_each(|&x|
        {
            data.iter().for_each(|&z|
                {
                    {
                        if x + z == 2020 {
                            a = x * z;
                        }
                    }
                    data.iter().for_each(
                        |&y|
                            {
                                if x + y + z == 2020 {
                                    b = x * y * z;
                                }
                            }
                    )
                }
            )
        }
    );
    println!("a = {}, b = {}", a, b);
    Ok(())
}
