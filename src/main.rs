use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{


    let data = read_file("input/1.txt")?;
    data.iter().for_each(|&x|
        {
            data.iter().for_each(
                |&y|
                    {
                        // println!( "{}", y+x);
                        if x + y == 2020 {
                            println!("{} + {} = {}", x, y, x + y);
                            println!("{} * {} = {}", x, y, x * y);
                            return;
                        }
                    }
            )
        }


    );

    Ok(())
}


//noinspection RsUnresolvedMethod
fn read_file(path: &str) -> Result<Vec<i32>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data: Vec<i32> = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        data.push(line.parse().unwrap());
    }
    Ok(data)
}

