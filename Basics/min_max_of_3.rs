use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line( &mut input ).unwrap();
    
    let mut nums = input.split_whitespace();
    
    let a : i32 = nums.next().unwrap().parse().unwrap();
    let b : i32 = nums.next().unwrap().parse().unwrap();
    let c : i32 = nums.next().unwrap().parse().unwrap();
    
     let mut min = a;
    if b < min {
        min = b;
    }
    if c < min {
        min = c;
    }

    let mut max = a;
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }
    
    println!("Min = {}", min);
    println!("Max = {}", max);
}