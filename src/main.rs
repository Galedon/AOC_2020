

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

    println!("day 7:");
    println!("{}", solutions::get_day_7()?);
    println!();

    Ok(())
}
