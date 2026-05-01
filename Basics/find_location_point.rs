use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let x: i64 = values.next().unwrap().parse().unwrap();
    let y: i64 = values.next().unwrap().parse().unwrap();

    if x == 0 && y == 0 {
        println!("Origin");
    } else if y == 0 {
        println!("X axis");
    } else if x == 0 {
        println!("Y axis");
    } else if x > 0 && y > 0 {
        println!("1st Quadrant");
    } else if x < 0 && y > 0 {
        println!("2nd Quadrant");
    } else if x < 0 && y < 0 {
        println!("3rd Quadrant");
    } else {
        println!("4th Quadrant");
    }
}