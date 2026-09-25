// use std::collections::{HashSet};
// use std::cmp::min;
// use crate::solutions::{read_lines_to_str, read_file_to_int, read_file_to_int_64};

use crate::solutions::read_lines_to_str;

mod solutions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_lines_to_str("input/11.txt")?;
    let mut array: Vec<Vec<char>> = data.iter().map(|s| s.chars().collect()).collect();
    let mut array_old = array.clone();

    fn get_occupied_neighbours(x: i32, y: i32, array_old: &Vec<Vec<char>>) -> i32 {
        let mut n_occ = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if (x + i >= array_old.len() as i32)
                    || (x + i < 0)
                    || (y + j >= array_old[0].len() as i32)
                    || (y + j < 0)
                {
                    continue;
                }
                if array_old[(x + i) as usize][(y + j) as usize] == '#' {
                    n_occ += 1;
                }
            }
        }
        n_occ
    }

    fn get_occupied_visible(x: i32, y: i32, array_old: &Vec<Vec<char>>) -> i32 {
        let mut n_occ = 0;
        for dir in [[1,0], [1,1], [0,1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]]{
            let i = dir[0];
            let j = dir[1];
            let mut k = 1;
            loop{
                if (x + k*i >= array_old.len() as i32)
                    || (x + k*i < 0)
                    || (y + k*j >= array_old[0].len() as i32)
                    || (y + k*j < 0)
                {break}
                if array_old[(x + k*i) as usize][(y + k*j) as usize] == '#'{
                    n_occ += 1;
                    break;
                }
                if array_old[(x + k*i) as usize][(y + k*j) as usize] == 'L'{break}
                k += 1;
            }
        }
        n_occ
    }

    fn one_seat(x: usize, y: usize, array: &mut Vec<Vec<char>>, array_old: &Vec<Vec<char>>) -> i32 {
        let mut n_changed = 0;
        if (array_old[x][y] == 'L') && (get_occupied_visible(x as i32, y as i32, array_old) == 0)
        {
            array[x][y] = '#';
            n_changed += 1;
        } else if (array_old[x][y] == '#')
            && (get_occupied_visible(x as i32, y as i32, array_old) >= 5)
        {
            array[x][y] = 'L';
            n_changed += 1
        }
        n_changed
    }

    fn one_iteration(
        array: &mut Vec<Vec<char>>,
        array_old: &mut Vec<Vec<char>>,
    ) -> i32 {
        let mut n_changed = 0;
        for i in 0..array_old.len() {
            for j in 0..array_old[i].len() {
                n_changed += one_seat(i, j, array, array_old);
            }
        }
        n_changed
    }

    let mut iteration = 0;
    loop {
        iteration += 1;
        println!("{iteration}");
        let n_changes = one_iteration(&mut array, &mut array_old);
        if n_changes == 0 {
            break;
        }
        array_old = array.clone();
    }

    let mut n_taken = 0;
    for row in array {
        for seat in row {
            if seat == '#' { n_taken += 1; }
        }
    }
    println!("n_taken: {}", n_taken);

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

    // println!("day 9:");
    // println!("{}", solutions::get_day_9()?);
    // println!();

    // println!("day 10:");
    // println!("{}", solutions::get_day_10()?);
    // println!();

    Ok(())
}
