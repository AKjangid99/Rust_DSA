use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let num1: i64 = input1.trim().parse().unwrap();
    let num2: i64 = input2.trim().parse().unwrap();

    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);
    println!("{} / {} = {}", num1, num2, num1 / num2);
    println!("{} % {} = {}", num1, num2, num1 % num2);
}