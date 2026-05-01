use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line( &mut input ).unwrap();
    
    let mut nums = input.split_whitespace();
    
    let num1 : i32 = nums.next().unwrap().parse().unwrap();
    let num2 : i32 = nums.next().unwrap().parse().unwrap();
    
    if num1 < num2 {
        println!("Min = {}" , num1);
        println!("Max = {}" , num2);
    }else {
        println!("Min = {}" , num2);
        println!("Max = {}" , num1);
    }
}