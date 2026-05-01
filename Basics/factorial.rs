use std::io;

fn main() {

    let mut input = String::new();
    
    io::stdin().read_line( &mut input).unwrap();
    
    let mut nums = input.split_whitespace();
    
    let num1 : i32 = nums.next().unwrap().parse().unwrap();
    let num2 : i32 = nums.next().unwrap().parse().unwrap();
    
    if num1%num2 == 0 {
        println!("Yes");
    }else {
        println!("No");
    }

}